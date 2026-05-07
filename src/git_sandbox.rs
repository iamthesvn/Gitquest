// git_sandbox.rs — Run real git commands in isolated temp repositories

use std::{
    fs,
    path::PathBuf,
    process::Command,
};

/// An isolated temporary Git repository for verifying player commands.
pub struct GitSandbox {
    pub path: PathBuf,
}

#[allow(dead_code)]
impl GitSandbox {
    /// Create a new sandbox in a fresh temp directory.
    pub fn new() -> Result<Self, String> {
        let base = std::env::temp_dir().join("gitquest-sandbox");
        fs::create_dir_all(&base).map_err(|e| format!("mkdir: {e}"))?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = base.join(format!("repo-{}-{}", std::process::id(), now));
        fs::create_dir_all(&path).map_err(|e| format!("mkdir repo: {e}"))?;

        Ok(Self { path })
    }

    /// Run a git command and return (stdout, stderr, exit_code).
    pub fn git(&self, args: &[&str]) -> (String, String, i32) {
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.path)
            .env("HOME", &self.path)
            .output();

        match output {
            Ok(o) => {
                let stdout = String::from_utf8_lossy(&o.stdout).to_string();
                let stderr = String::from_utf8_lossy(&o.stderr).to_string();
                let code = o.status.code().unwrap_or(-1);
                (stdout, stderr, code)
            }
            Err(e) => (String::new(), format!("Failed to run git: {e}"), -1),
        }
    }

    /// Run a shell command (non-git) in the sandbox.
    pub fn sh(&self, cmd: &str) -> (String, String, i32) {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", cmd]).current_dir(&self.path).env("HOME", &self.path).output()
        } else {
            Command::new("sh").args(["-c", cmd]).current_dir(&self.path).env("HOME", &self.path).output()
        };

        match output {
            Ok(o) => {
                let stdout = String::from_utf8_lossy(&o.stdout).to_string();
                let stderr = String::from_utf8_lossy(&o.stderr).to_string();
                let code = o.status.code().unwrap_or(-1);
                (stdout, stderr, code)
            }
            Err(e) => (String::new(), format!("Failed to run shell: {e}"), -1),
        }
    }

    /// Write a file into the sandbox.
    pub fn write_file(&self, name: &str, content: &str) -> Result<(), String> {
        let p = self.path.join(name);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("mkdir: {e}"))?;
        }
        fs::write(&p, content).map_err(|e| format!("write: {e}"))
    }

    /// Check if a path exists (file or dir).
    pub fn exists(&self, rel: &str) -> bool {
        self.path.join(rel).exists()
    }

    /// Check if a file is tracked/staged.
    pub fn is_staged(&self, rel: &str) -> bool {
        let (out, _, code) = self.git(&["diff", "--cached", "--name-only"]);
        code == 0 && out.lines().any(|l| l.trim() == rel)
    }

    /// Check if a file has unstaged modifications.
    pub fn is_modified(&self, rel: &str) -> bool {
        let (out, _, code) = self.git(&["diff", "--name-only"]);
        code == 0 && out.lines().any(|l| l.trim() == rel)
    }

    /// Check if a branch exists.
    pub fn has_branch(&self, name: &str) -> bool {
        let (out, _, code) = self.git(&["branch", "--list", name]);
        code == 0 && out.contains(name)
    }

    /// Current branch name.
    pub fn current_branch(&self) -> Option<String> {
        let (out, _, code) = self.git(&["branch", "--show-current"]);
        if code != 0 {
            return None;
        }
        let name = out.trim();
        if name.is_empty() { None } else { Some(name.to_string()) }
    }

    /// Check if HEAD points to a commit with the given message (substring match).
    pub fn head_commit_message_contains(&self, needle: &str) -> bool {
        let (out, _, code) = self.git(&["log", "-1", "--pretty=%B"]);
        code == 0 && out.contains(needle)
    }

    /// Number of commits on current branch.
    pub fn commit_count(&self) -> usize {
        let (out, _, code) = self.git(&["rev-list", "--count", "HEAD"]);
        if code != 0 {
            return 0;
        }
        out.trim().parse().unwrap_or(0)
    }

    /// Check if a remote exists.
    pub fn has_remote(&self, name: &str) -> bool {
        let (out, _, code) = self.git(&["remote"]);
        code == 0 && out.lines().any(|l| l.trim() == name)
    }

    /// Read stdout of a command.
    pub fn stdout(&self, args: &[&str]) -> String {
        self.git(args).0
    }
}

impl Drop for GitSandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandbox_init_and_branch() {
        let sb = GitSandbox::new().unwrap();
        assert!(sb.path.exists());

        let (_, err, code) = sb.git(&["init"]);
        assert_eq!(code, 0, "git init failed: {err}");
        assert!(sb.exists(".git"));

        let (_, err2, code2) = sb.git(&["checkout", "-b", "test-branch"]);
        assert_eq!(code2, 0, "checkout failed: {err2}");
        assert_eq!(sb.current_branch(), Some("test-branch".to_string()));
    }
}
