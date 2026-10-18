use std::fs;
use std::path::Path;

fn semver(value: &str, prerelease: bool) -> bool {
    let (core, suffix) = match value.split_once('-') {
        Some((core, suffix)) if prerelease => (core, Some(suffix)),
        Some(_) => return false,
        None => (value, None),
    };
    let mut components = core.split('.');
    let core_ok = (0..3).all(|_| {
        components
            .next()
            .is_some_and(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))
    }) && components.next().is_none();
    core_ok
        && suffix.is_none_or(|s| {
            !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'.')
        })
}

pub fn check(root: &Path, reference: &str) -> Result<(String, Vec<String>), String> {
    let version_path = root.join("docs/VERSION");
    let version = fs::read_to_string(&version_path)
        .map_err(|e| format!("{}: {e}", version_path.display()))?
        .trim()
        .to_owned();
    let mut errors = Vec::new();
    if !semver(&version, true) {
        errors.push(format!("docs/VERSION {version:?} is not semver"));
    }

    let whitepaper_path = root.join("docs/WHITEPAPER.md");
    let whitepaper = fs::read_to_string(&whitepaper_path)
        .map_err(|e| format!("{}: {e}", whitepaper_path.display()))?;
    if !whitepaper
        .chars()
        .take(2000)
        .collect::<String>()
        .contains(&format!("Version {version}"))
    {
        errors.push(format!("WHITEPAPER header lacks \"Version {version}\""));
    }

    if let Some(tag) = reference.strip_prefix("refs/tags/v") {
        if semver(tag, false) && tag != version {
            errors.push(format!("tag v{tag} != docs/VERSION {version}"));
        }
    }
    if reference.starts_with("refs/heads/release/") && version.contains("-dev") {
        errors.push(format!("release branch with -dev VERSION {version}"));
    }
    Ok((version, errors))
}

pub fn run(root: &Path, reference: &str) -> i32 {
    match check(root, reference) {
        Ok((version, errors)) => {
            println!(
                "VERSION={version} ref={}",
                if reference.is_empty() {
                    "(local)"
                } else {
                    reference
                }
            );
            for error in &errors {
                println!("ERROR: {error}");
            }
            i32::from(!errors.is_empty())
        }
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

#[cfg(all(standalone_version_check, not(test)))]
fn main() {
    let reference = std::env::var("GITHUB_REF").unwrap_or_default();
    std::process::exit(run(Path::new("."), &reference));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    struct Fixture(std::path::PathBuf);
    impl Fixture {
        fn new(version: &str, whitepaper: &str) -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let root = std::env::temp_dir().join(format!(
                "jarambit-version-check-{}-{nonce}",
                std::process::id()
            ));
            fs::create_dir_all(root.join("docs")).unwrap();
            fs::write(root.join("docs/VERSION"), version).unwrap();
            fs::write(root.join("docs/WHITEPAPER.md"), whitepaper).unwrap();
            Self(root)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn accepts_matching_local_version_and_header() {
        let f = Fixture::new("1.2.3\n", "# Whitepaper\nVersion 1.2.3\n");
        assert!(check(&f.0, "").unwrap().1.is_empty());
    }

    #[test]
    fn rejects_bad_version_header_tag_and_release_prerelease() {
        let f = Fixture::new("1.2.3-dev\n", "# Whitepaper\n");
        let errors = check(&f.0, "refs/tags/v2.0.0").unwrap().1;
        assert_eq!(
            errors,
            [
                "WHITEPAPER header lacks \"Version 1.2.3-dev\"",
                "tag v2.0.0 != docs/VERSION 1.2.3-dev",
            ]
        );
        assert!(check(&f.0, "refs/heads/release/1.x")
            .unwrap()
            .1
            .iter()
            .any(|e| e.contains("release branch with -dev")));
    }

    #[test]
    fn allows_legacy_non_semver_tags_and_checks_only_first_2000_chars() {
        let f = Fixture::new("1.2.3\n", &format!("{}Version 1.2.3", "x".repeat(2000)));
        assert_eq!(
            check(&f.0, "refs/tags/v0.1.0-g3neckrail").unwrap().1.len(),
            1
        );
    }
}
