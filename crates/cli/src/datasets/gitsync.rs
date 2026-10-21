//! Commit (and optionally push) record shards with the `git` CLI.
//!
//! Only the given paths are staged and committed (`git commit --only -- <paths>`), so unrelated
//! staged or unstaged work in the same checkout is never swept into a data commit. Record shards
//! live in run-unique directories, so concurrent hosts never touch the same file and a rebase onto
//! the remote data branch cannot conflict. Pushing rebases only when the checkout is otherwise clean.
use std::path::Path;
use std::process::Command;
use std::time::Duration;

fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git").current_dir(root).args(args).output().map_err(|e| format!("git {}: {e}", args.join(" ")))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        Err(format!("git {} failed: {}", args.join(" "), String::from_utf8_lossy(&out.stderr).trim()))
    }
}

/// Retry transient index-lock contention (another git process in the same checkout).
fn git_retry(root: &Path, args: &[&str]) -> Result<String, String> {
    let mut last = String::new();
    for attempt in 0..6 {
        match git(root, args) {
            Ok(s) => return Ok(s),
            Err(e) if e.contains("index.lock") => {
                last = e;
                std::thread::sleep(Duration::from_millis(250 << attempt));
            }
            Err(e) => return Err(e),
        }
    }
    Err(last)
}

pub fn is_repo(root: &Path) -> bool {
    git(root, &["rev-parse", "--is-inside-work-tree"]).map(|s| s == "true").unwrap_or(false)
}

pub fn head(root: &Path) -> String {
    git(root, &["rev-parse", "--short=12", "HEAD"]).unwrap_or_else(|_| "unknown".into())
}

/// Stage and commit exactly `paths` (relative to `root`). Returns Ok(None) when nothing changed.
pub fn commit_paths(root: &Path, paths: &[String], message: &str) -> Result<Option<String>, String> {
    if paths.is_empty() {
        return Ok(None);
    }
    let mut add: Vec<&str> = vec!["add", "--"];
    add.extend(paths.iter().map(String::as_str));
    git_retry(root, &add)?;
    let mut diff: Vec<&str> = vec!["diff", "--cached", "--name-only", "--"];
    diff.extend(paths.iter().map(String::as_str));
    if git(root, &diff)?.is_empty() {
        return Ok(None);
    }
    let mut commit: Vec<&str> = vec!["commit", "--no-verify", "--quiet", "-m", message, "--only", "--"];
    commit.extend(paths.iter().map(String::as_str));
    git_retry(root, &commit)?;
    Ok(Some(head(root)))
}

/// Push HEAD to `remote` `branch`, rebasing onto the remote branch on rejection (only when the
/// checkout has no other tracked modifications). Up to `attempts` tries with backoff.
pub fn push(root: &Path, remote: &str, branch: &str, attempts: usize) -> Result<(), String> {
    let refspec = format!("HEAD:refs/heads/{branch}");
    let mut last = String::new();
    for attempt in 0..attempts.max(1) {
        match git(root, &["push", "--quiet", remote, &refspec]) {
            Ok(_) => return Ok(()),
            Err(e) => last = e,
        }
        let dirty = git(root, &["status", "--porcelain", "--untracked-files=no"])?;
        if !dirty.is_empty() {
            return Err(format!("push rejected and the checkout has other tracked changes; not rebasing automatically: {last}"));
        }
        if git(root, &["fetch", "--quiet", remote, branch]).is_ok() {
            git(root, &["rebase", "--quiet", "FETCH_HEAD"]).map_err(|e| {
                let _ = git(root, &["rebase", "--abort"]);
                format!("rebase onto {remote}/{branch} failed: {e}")
            })?;
        }
        std::thread::sleep(Duration::from_secs(2u64 << attempt));
    }
    Err(last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commits_only_the_requested_paths() {
        let root = std::env::temp_dir().join(format!("jarambit-gitsync-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("datasets/records/run")).unwrap();
        if git(&root, &["init", "--quiet"]).is_err() {
            return; // git unavailable on this host
        }
        let _ = git(&root, &["config", "user.email", "ci@example.invalid"]);
        let _ = git(&root, &["config", "user.name", "ci"]);
        std::fs::write(root.join("unrelated.txt"), "x").unwrap();
        git(&root, &["add", "unrelated.txt"]).unwrap();
        std::fs::write(root.join("datasets/records/run/r00.s1.csv"), "a\n1\n").unwrap();
        let sha = commit_paths(&root, &["datasets/records/run".into()], "data: test").unwrap();
        assert!(sha.is_some());
        let files = git(&root, &["show", "--name-only", "--format=", "HEAD"]).unwrap();
        assert_eq!(files, "datasets/records/run/r00.s1.csv");
        // the unrelated staged file is still staged and uncommitted
        assert_eq!(git(&root, &["diff", "--cached", "--name-only"]).unwrap(), "unrelated.txt");
        assert_eq!(commit_paths(&root, &["datasets/records/run".into()], "data: noop").unwrap(), None);
        assert!(is_repo(&root));
    }
}
