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
            let _ = sb.write_file("README.md", "# Hello");
            let _ = sb.git(&["add", "README.md"]);
            let _ = sb.git(&["commit", "-m", "init"]);
        },
        verify: |sb| sb.has_branch("feature"),
    }
}
