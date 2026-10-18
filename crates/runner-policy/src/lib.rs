use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum Node {
    Scalar(String),
    Sequence(Vec<Node>),
    Mapping(BTreeMap<String, Node>),
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NodeVisitor;
        impl<'de> Visitor<'de> for NodeVisitor {
            type Value = Node;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a YAML scalar, sequence, or mapping with unique string keys")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Node, E> {
                Ok(Node::Scalar(value.to_string()))
            }
            fn visit_i64<E>(self, value: i64) -> Result<Node, E> {
                Ok(Node::Scalar(value.to_string()))
            }
            fn visit_u64<E>(self, value: u64) -> Result<Node, E> {
                Ok(Node::Scalar(value.to_string()))
            }
            fn visit_f64<E>(self, value: f64) -> Result<Node, E> {
                Ok(Node::Scalar(value.to_string()))
            }
            fn visit_str<E>(self, value: &str) -> Result<Node, E> {
                Ok(Node::Scalar(value.to_owned()))
            }
            fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Node, E> {
                Ok(Node::Scalar(value.to_owned()))
            }
            fn visit_string<E>(self, value: String) -> Result<Node, E> {
                Ok(Node::Scalar(value))
            }
            fn visit_unit<E>(self) -> Result<Node, E> {
                Ok(Node::Scalar("null".into()))
            }
            fn visit_none<E>(self) -> Result<Node, E> {
                Ok(Node::Scalar("null".into()))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Node, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element()? {
                    values.push(value);
                }
                Ok(Node::Sequence(values))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Node, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut values = BTreeMap::new();
                while let Some((key, value)) = map.next_entry::<String, Node>()? {
                    if values.insert(key.clone(), value).is_some() {
                        return Err(de::Error::custom(format!(
                            "duplicate or invalid YAML key: {key:?}"
                        )));
                    }
                }
                Ok(Node::Mapping(values))
            }
        }
        deserializer.deserialize_any(NodeVisitor)
    }
}

fn reusable_path_is_local(target: &str, root: &Path) -> bool {
    let Some(name) = target.strip_prefix("./.github/workflows/") else {
        return false;
    };
    if name.is_empty()
        || !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))
    {
        return false;
    }
    if !(name.ends_with(".yml") || name.ends_with(".yaml")) {
        return false;
    }
    root.join(target).is_file()
}

fn workflow_errors(document: &Node, root: &Path) -> Vec<String> {
    let Node::Mapping(document) = document else {
        return vec!["workflow must define a nonempty jobs mapping".into()];
    };
    let Some(Node::Mapping(jobs)) = document.get("jobs") else {
        return vec!["workflow must define a nonempty jobs mapping".into()];
    };
    if jobs.is_empty() {
        return vec!["workflow must define a nonempty jobs mapping".into()];
    }

    let required = ["self-hosted", "xbabe", "general"];
    let mut errors = Vec::new();
    for (name, job) in jobs {
        let Node::Mapping(job) = job else {
            errors.push(format!("{name}: job must be a mapping"));
            continue;
        };
        if job.contains_key("uses") {
            let target_is_valid = match job.get("uses") {
                Some(Node::Scalar(target)) => reusable_path_is_local(target, root),
                _ => false,
            };
            if !target_is_valid {
                errors.push(format!(
                    "{name}: reusable workflows must exist locally so their runners can be audited"
                ));
            }
            if job.contains_key("runs-on") {
                errors.push(format!(
                    "{name}: reusable workflow calls cannot define runs-on"
                ));
            }
            continue;
        }
        let Some(Node::Sequence(labels)) = job.get("runs-on") else {
            errors.push(format!("{name}: runs-on must be an explicit label list containing self-hosted, xbabe, general"));
            continue;
        };
        if labels.is_empty()
            || labels
                .iter()
                .any(|label| !matches!(label, Node::Scalar(value) if !value.trim().is_empty()))
        {
            errors.push(format!("{name}: runs-on must be an explicit label list containing self-hosted, xbabe, general"));
            continue;
        }
        let actual: BTreeSet<String> = labels
            .iter()
            .filter_map(|label| match label {
                Node::Scalar(value) => Some(value.to_lowercase()),
                _ => None,
            })
            .collect();
        let missing: Vec<_> = required
            .iter()
            .filter(|label| !actual.contains(**label))
            .copied()
            .collect();
        if !missing.is_empty() {
            errors.push(format!(
                "{name}: missing local runner labels: {}",
                missing.join(", ")
            ));
        }
    }
    errors
}

pub fn check_repository(root: &Path) -> Vec<String> {
    let directory = root.join(".github/workflows");
    let entries = match fs::read_dir(&directory) {
        Ok(entries) => entries,
        Err(e) => return vec![format!(".github/workflows: {e}")],
    };
    let mut workflows: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "yml" || x == "yaml"))
        .collect();
    workflows.sort();
    if workflows.is_empty() {
        return vec!["no GitHub Actions workflows found".into()];
    }
    let mut errors = Vec::new();
    for path in workflows {
        let relative = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .display()
            .to_string();
        let findings = match fs::read_to_string(&path) {
            Err(e) => vec![e.to_string()],
            Ok(text) => match serde_yaml::from_str::<Node>(&text) {
                Err(e) => vec![e.to_string()],
                Ok(doc) => workflow_errors(&doc, root),
            },
        };
        errors.extend(
            findings
                .into_iter()
                .map(|finding| format!("{relative}: {finding}")),
        );
    }
    errors
}

pub fn run(root: &Path) -> i32 {
    let errors = check_repository(root);
    if errors.is_empty() {
        println!(
            "Runner policy passed: every workflow job requires self-hosted + xbabe + general."
        );
        0
    } else {
        for error in errors {
            eprintln!("::error::{error}");
        }
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    struct Fixture(PathBuf);
    impl Fixture {
        fn new(content: &str, reusable: Option<&str>) -> Self {
            let nonce = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!(
                "jarambit-runner-policy-{}-{nonce}",
                std::process::id()
            ));
            let workflows = root.join(".github/workflows");
            fs::create_dir_all(&workflows).unwrap();
            fs::write(workflows.join("ci.yml"), content).unwrap();
            if let Some(reusable) = reusable {
                fs::write(workflows.join("reusable.yaml"), reusable).unwrap();
            }
            Self(root)
        }
        fn errors(&self) -> Vec<String> {
            check_repository(&self.0)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn general_pool_with_capability_and_case_insensitive_labels() {
        let f = Fixture::new(
            "jobs:\n  test:\n    runs-on: [self-hosted, XBabe, GENERAL, linux, jarambit-d1]\n",
            None,
        );
        assert!(f.errors().is_empty());
    }
    #[test]
    fn matrix_cannot_remove_required_labels() {
        let f = Fixture::new(
            "jobs:\n  test:\n    runs-on: [self-hosted, xbabe, general, '${{ matrix.host }}']\n",
            None,
        );
        assert!(f.errors().is_empty());
    }
    #[test]
    fn github_hosted_is_rejected() {
        let f = Fixture::new("jobs:\n  test:\n    runs-on: ubuntu-latest\n", None);
        assert!(!f.errors().is_empty());
    }
    #[test]
    fn self_hosted_outside_general_pool_is_rejected() {
        for labels in [
            "self-hosted, xbabe",
            "self-hosted, general",
            "xbabe, general",
        ] {
            let f = Fixture::new(&format!("jobs:\n  test:\n    runs-on: [{labels}]\n"), None);
            assert!(!f.errors().is_empty());
        }
    }
    #[test]
    fn dynamic_runs_on_and_runner_group_mapping_are_rejected() {
        for value in ["'${{ matrix.runner }}'", "{group: general}"] {
            let f = Fixture::new(&format!("jobs:\n  test:\n    runs-on: {value}\n"), None);
            assert!(!f.errors().is_empty());
        }
    }
    #[test]
    fn remote_reusable_workflow_is_rejected() {
        let f = Fixture::new(
            "jobs:\n  test:\n    uses: other/repo/.github/workflows/ci.yml@main\n",
            None,
        );
        assert!(!f.errors().is_empty());
    }
    #[test]
    fn local_reusable_workflow_is_also_audited() {
        let caller = "jobs:\n  test:\n    uses: ./.github/workflows/reusable.yaml\n";
        let f = Fixture::new(
            caller,
            Some("jobs:\n  test:\n    runs-on: [self-hosted, xbabe, general]\n"),
        );
        assert!(f.errors().is_empty());
        let f = Fixture::new(caller, Some("jobs:\n  test:\n    runs-on: ubuntu-latest\n"));
        assert!(!f.errors().is_empty());
        let f = Fixture::new(caller, None);
        assert!(!f.errors().is_empty());
    }
    #[test]
    fn duplicate_routing_or_job_keys_are_rejected() {
        for text in [
            "jobs:\n  test:\n    runs-on: ubuntu-latest\n    runs-on: [self-hosted, xbabe, general]\n",
            "jobs: {test: {runs-on: ubuntu-latest}, test: {runs-on: [self-hosted, xbabe, general]}}\n",
        ] {
            let f = Fixture::new(text, None);
            assert!(!f.errors().is_empty());
        }
    }
    #[test]
    fn empty_invalid_and_yaml_suffix_workflows_are_checked() {
        for content in [
            "",
            "jobs: {}",
            "jobs: [test]",
            "jobs: {test: []}",
            "jobs: {",
        ] {
            let f = Fixture::new(content, None);
            assert!(!f.errors().is_empty());
        }
        let f = Fixture::new(
            "jobs:\n  test:\n    runs-on: [self-hosted, xbabe, general]\n",
            Some("jobs:\n  test:\n    runs-on: windows-latest\n"),
        );
        assert!(!f.errors().is_empty());
    }
}
