// gitlings/exercises.rs — Gitlings exercise definitions
// Pure rustlings-style: no narrative, just instructions + real git execution.

use crate::git_sandbox::GitSandbox;

#[allow(dead_code)]
pub struct Exercise {
    pub name: &'static str,
    pub description: &'static str,
    pub hint: &'static str,
    pub setup: fn(&mut GitSandbox),
    pub verify: fn(&GitSandbox) -> bool,
}

pub fn all_exercises() -> Vec<Exercise> {
    vec![
        exercise_init(),
        exercise_config(),
        exercise_add(),
        exercise_commit(),
        exercise_branch(),
    ]
}

fn exercise_init() -> Exercise {
    Exercise {
        name: "01_git_init",
        description: "Create a new git repository in the current directory.",
        hint: "The command is two words: git init",
        setup: |_sb| {},
        verify: |sb| sb.exists(".git"),
    }
}

fn exercise_config() -> Exercise {
    Exercise {
        name: "02_git_config",
        description: "Set the global git user name to 'Alex Chen'.",
        hint: "git config --global user.name \"Alex Chen\"",
        setup: |sb| { let _ = sb.git(&["init"]); },
        verify: |sb| sb.stdout(&["config", "user.name"]).contains("Alex Chen"),
    }
}

fn exercise_add() -> Exercise {
    Exercise {
        name: "03_git_add",
        description: "Stage the file README.md for the next commit.",
        hint: "git add README.md",
        setup: |sb| {
            let _ = sb.git(&["init"]);
            let _ = sb.write_file("README.md", "# Hello");
        },
        verify: |sb| sb.is_staged("README.md"),
    }
}

fn exercise_commit() -> Exercise {
    Exercise {
        name: "04_git_commit",
        description: "Commit the staged changes with the message 'First commit'.",
        hint: "git commit -m \"First commit\"",
        setup: |sb| {
            let _ = sb.git(&["init"]);
            let _ = sb.git(&["config", "user.name", "Test"]);
            let _ = sb.git(&["config", "user.email", "test@example.com"]);
            let _ = sb.write_file("README.md", "# Hello");
            let _ = sb.git(&["add", "README.md"]);
        },
        verify: |sb| sb.commit_count() >= 1 && sb.head_commit_message_contains("First commit"),
    }
}

fn exercise_branch() -> Exercise {
    Exercise {
        name: "05_git_branch",
        description: "Create a new branch called 'feature'.",
        hint: "git branch feature",
        setup: |sb| {
            let _ = sb.git(&["init"]);
            let _ = sb.git(&["config", "user.name", "Test"]);
            let _ = sb.git(&["config", "user.email", "test@example.com"]);
            let _ = sb.write_file("README.md", "# Hello");
            let _ = sb.git(&["add", "README.md"]);
            let _ = sb.git(&["commit", "-m", "init"]);
        },
        verify: |sb| sb.has_branch("feature"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_sandbox::GitSandbox;

    fn run_exercise(ex: &Exercise, cmd: &str) -> (bool, String) {
        let mut sb = GitSandbox::new().unwrap();
        (ex.setup)(&mut sb);
        let (out, err, code) = sb.sh(cmd);
        let output = if code != 0 {
            if err.is_empty() { out } else { err }
        } else {
            if out.is_empty() { "(no output)".to_string() } else { out }
        };
        let pass = code == 0 && (ex.verify)(&sb);
        (pass, output)
    }

    #[test]
    fn ex01_init_correct() {
        let ex = exercise_init();
        let (pass, out) = run_exercise(&ex, "git init");
        assert!(pass, "git init should pass: {}", out);
    }

    #[test]
    fn ex01_init_wrong_command() {
        let ex = exercise_init();
        let (pass, out) = run_exercise(&ex, "git status");
        assert!(!pass, "git status should fail before init: {}", out);
    }

    #[test]
    fn ex02_config_correct() {
        let ex = exercise_config();
        let (pass, out) = run_exercise(&ex, "git config --global user.name \"Alex Chen\"");
        assert!(pass, "git config should pass: {}", out);
    }

    #[test]
    fn ex02_config_wrong_value() {
        let ex = exercise_config();
        let (pass, _out) = run_exercise(&ex, "git config --global user.name \"Wrong Name\"");
        assert!(!pass, "wrong name should fail");
    }

    #[test]
    fn ex03_add_correct() {
        let ex = exercise_add();
        let (pass, out) = run_exercise(&ex, "git add README.md");
        assert!(pass, "git add should pass: {}", out);
    }

    #[test]
    fn ex03_add_wrong_file() {
        let ex = exercise_add();
        let (pass, out) = run_exercise(&ex, "git add wrong.txt");
        assert!(!pass, "adding wrong file should fail: {}", out);
    }

    #[test]
    fn ex04_commit_correct() {
        let ex = exercise_commit();
        let (pass, out) = run_exercise(&ex, "git commit -m \"First commit\"");
        assert!(pass, "git commit should pass: {}", out);
    }

    #[test]
    fn ex04_commit_wrong_message() {
        let ex = exercise_commit();
        let (pass, _out) = run_exercise(&ex, "git commit -m \"Wrong message\"");
        assert!(!pass, "wrong message should fail");
    }

    #[test]
    fn ex05_branch_correct() {
        let ex = exercise_branch();
        let (pass, out) = run_exercise(&ex, "git branch feature");
        assert!(pass, "git branch should pass: {}", out);
    }

    #[test]
    fn ex05_branch_wrong_name() {
        let ex = exercise_branch();
        let (pass, _out) = run_exercise(&ex, "git branch wrong-name");
        assert!(!pass, "wrong branch name should fail");
    }

    #[test]
    fn all_exercises_load() {
        let exercises = all_exercises();
        assert_eq!(exercises.len(), 5);
        assert_eq!(exercises[0].name, "01_git_init");
        assert_eq!(exercises[4].name, "05_git_branch");
    }
}
