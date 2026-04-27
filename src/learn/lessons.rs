// learn/lessons.rs — Learn mode lesson content for GitQuest
// Covers Volume 1 basics: init, config, add, commit, push, branch

#[derive(Clone)]
pub struct LessonStep {
    pub title: &'static str,
    pub text: &'static str,
    pub command: Option<&'static str>,
    pub art_frames: &'static [&'static str],
    /// Frames shown after the user presses Enter to "run" the command.
    /// Empty for explanation-only steps.
    pub result_frames: &'static [&'static str],
}

#[derive(Clone)]
pub struct Lesson {
    pub title: &'static str,
    pub tagline: &'static str,
    pub steps: &'static [LessonStep],
}

pub fn all_lessons() -> Vec<Lesson> {
    vec![
        lesson_init(),
        lesson_config(),
        lesson_add(),
        lesson_commit(),
        lesson_push(),
        lesson_branch(),
        lesson_switch(),
        lesson_merge(),
        lesson_status(),
        lesson_log(),
        lesson_pull(),
    ]
}

fn lesson_init() -> Lesson {
    Lesson {
        title: "The Empty Desk",
        tagline: "Every project starts with a single command.",
        steps: &[
            LessonStep {
                title: "What is git init?",
                text: "Before Git can track your work, it needs a place to store its history. 'git init' creates a hidden .git/ folder inside your project directory. This folder is Git's memory.",
                command: None,
                art_frames: &[
                    concat!(
                        "                          \n",
                        "     📁 my-project/       \n",
                        "                          \n",
                        "     (empty folder)       \n",
                        "                          "
                    ),
                    concat!(
                        "                          \n",
                        "     📁 my-project/       \n",
                        "       └─ .git/  ← new!   \n",
                        "                          \n",
                        "     Repository born.     \n",
                        "                          "
                    ),
                    concat!(
                        "                          \n",
                        "     📁 my-project/       \n",
                        "       └─ .git/           \n",
                        "          ├─ HEAD         \n",
                        "          ├─ config       \n",
                        "          └─ objects/     \n",
                        "                          "
                    ),
                ],
                result_frames: &[],
            },
            LessonStep {
                title: "Run the command",
                text: "This creates the .git directory and turns the folder into a repository.",
                command: Some("git init"),
                art_frames: &[
                    concat!(
                        "                          \n",
                        "     📁 my-project/       \n",
                        "                          \n",
                        "     $ git init           \n",
                        "     Initialized repo.    \n",
                        "                          "
                    ),
                ],
                result_frames: &[
                    concat!(
                        "                          \n",
                        "     📁 my-project/       \n",
                        "       └─ .git/  ← new!   \n",
                        "                          \n",
                        "     Repository ready.    \n",
                        "                          "
                    ),
                    concat!(
                        "                          \n",
                        "     📁 my-project/       \n",
                        "       └─ .git/           \n",
                        "          ├─ HEAD         \n",
                        "          ├─ config       \n",
                        "          └─ objects/     \n",
                        "                          "
                    ),
                ],
            },
        ],
    }
}

fn lesson_config() -> Lesson {
    Lesson {
        title: "The Config Chaos",
        tagline: "Git needs to know who you are.",
        steps: &[
            LessonStep {
                title: "Identity matters",
                text: "Every commit records its author. Without configuration, your commits show up as 'unknown'. Teams need to know who wrote what. 'git config' sets your name and email globally.",
                command: None,
                art_frames: &[
                    concat!(
                        "  ┌────────────────────┐ \n",
                        "  │  commit a1b2c3d   │ \n",
                        "  │  Author: ???      │ \n",
                        "  │  Date: Mon 14 Apr │ \n",
                        "  │                   │ \n",
                        "  │  Who wrote this?! │ \n",
                        "  └────────────────────┘ "
                    ),
                    concat!(
                        "  ┌────────────────────┐ \n",
                        "  │  commit a1b2c3d   │ \n",
                        "  │  Author: Alex Chen│ \n",
                        "  │  Date: Mon 14 Apr │ \n",
                        "  │                   │ \n",
                        "  │  That's better.   │ \n",
                        "  └────────────────────┘ "
                    ),
                ],
                result_frames: &[],
            },
            LessonStep {
                title: "Run the command",
                text: "This sets your name globally so every commit you make is properly attributed.",
                command: Some("git config --global user.name \"Alex Chen\""),
                art_frames: &[
                    concat!(
                        "  ┌────────────────────┐ \n",
                        "  │  $ git config ...  │ \n",
                        "  │                    │ \n",
                        "  │  Setting name...   │ \n",
                        "  └────────────────────┘ "
                    ),
                ],
                result_frames: &[
                    concat!(
                        "  ┌────────────────────┐ \n",
                        "  │  ✓ Name set        │ \n",
                        "  │                    │ \n",
                        "  │  Alex Chen         │ \n",
                        "  └────────────────────┘ "
                    ),
                    concat!(
                        "  ┌────────────────────┐ \n",
                        "  │  commit a1b2c3d   │ \n",
                        "  │  Author: Alex Chen│ \n",
                        "  │  Date: Mon 14 Apr │ \n",
                        "  └────────────────────┘ "
                    ),
                ],
            },
        ],
    }
}

fn lesson_add() -> Lesson {
    Lesson {
        title: "The Missing Files",
        tagline: "Choose what matters. Leave the noise behind.",
        steps: &[
            LessonStep {
                title: "The staging area",
                text: "Git doesn't commit everything automatically. You deliberately choose which changes to include. The staging area is your curated selection of changes for the next commit.",
                command: None,
                art_frames: &[
                    concat!(
                        "  Working Dir          \n",
                        "  📄 index.html  [M]   \n",
                        "  📄 styles.css  [M]   \n",
                        "  🗑 .DS_Store   [?]   \n",
                        "                       \n",
                        "  Staging: [ EMPTY ]   "
                    ),
                    concat!(
                        "  Working Dir          \n",
                        "  📄 styles.css  [M]   \n",
                        "  🗑 .DS_Store   [?]   \n",
                        "                       \n",
                        "  Staging:             \n",
                        "  📄 index.html  ✓     "
                    ),
                    concat!(
                        "  Working Dir          \n",
                        "  🗑 .DS_Store   [?]   \n",
                        "                       \n",
                        "  Staging:             \n",
                        "  📄 index.html  ✓     \n",
                        "  📄 styles.css  ✓     "
                    ),
                ],
                result_frames: &[],
            },
            LessonStep {
                title: "Run the command",
                text: "This stages both files, preparing them to be committed. .DS_Store is left unstaged.",
                command: Some("git add index.html styles.css"),
                art_frames: &[
                    concat!(
                        "  Working Dir          \n",
                        "  📄 index.html  [M]   \n",
                        "  📄 styles.css  [M]   \n",
                        "  🗑 .DS_Store   [?]   \n",
                        "                       \n",
                        "  $ git add ...        \n",
                        "                       "
                    ),
                ],
                result_frames: &[
                    concat!(
                        "  Working Dir          \n",
                        "  🗑 .DS_Store   [?]   \n",
                        "                       \n",
                        "  Staging:             \n",
                        "  📄 index.html  ✓     \n",
                        "  📄 styles.css  ✓     \n",
                        "  ✓ Staged 2 files     "
                    ),
                ],
            },
        ],
    }
}

fn lesson_commit() -> Lesson {
    Lesson {
        title: "The First Commit",
        tagline: "A commit is a promise to the future.",
        steps: &[
            LessonStep {
                title: "What is a commit?",
                text: "A commit is a permanent snapshot of your staged changes, plus a message explaining what you did. It gets a unique hash (like a fingerprint) and becomes part of the project's history forever.",
                command: None,
                art_frames: &[
                    concat!(
                        "  Staging Area         \n",
                        "  📄 index.html  ✓     \n",
                        "  📄 styles.css  ✓     \n",
                        "                       \n",
                        "  (not yet permanent)  "
                    ),
                    concat!(
                        "  Staging Area         \n",
                        "  📄 index.html  ✓     \n",
                        "  📄 styles.css  ✓     \n",
                        "                       \n",
                        "  Committing...        "
                    ),
                    concat!(
                        "                       \n",
                        "  main                 \n",
                        "    ●──●──●            \n",
                        "   c1 c2  c3           \n",
                        "            ↑          \n",
                        "         HEAD          \n",
                        "                       \n",
                        "  'Add landing page'   "
                    ),
                ],
                result_frames: &[],
            },
            LessonStep {
                title: "Run the command",
                text: "This creates a new commit with your message, permanently saving the staged changes.",
                command: Some("git commit -m \"Add landing page styles and layout\""),
                art_frames: &[
                    concat!(
                        "  main                 \n",
                        "    ●──●               \n",
                        "   c1  c2              \n",
                        "        ↑              \n",
                        "       HEAD            \n",
                        "                       \n",
                        "  $ git commit -m ...  "
                    ),
                ],
                result_frames: &[
                    concat!(
                        "                       \n",
                        "  main                 \n",
                        "    ●──●──●            \n",
                        "   c1 c2  c3           \n",
                        "            ↑          \n",
                        "         HEAD          \n",
                        "                       \n",
                        "  ✓ Committed c3       "
                    ),
                    concat!(
                        "  Staging Area         \n",
                        "  [ EMPTY ]            \n",
                        "                       \n",
                        "  main                 \n",
                        "    ●──●──●            \n",
                        "   c1 c2  c3           \n",
                        "            ↑          \n",
                        "         HEAD          "
                    ),
                ],
            },
        ],
    }
}

fn lesson_push() -> Lesson {
    Lesson {
        title: "The Repo Goes Remote",
        tagline: "Your work deserves to outlast your laptop.",
        steps: &[
            LessonStep {
                title: "What is push?",
                text: "'git push' uploads your local commits to a remote repository (like GitHub). This backs up your work and lets teammates see it. 'origin' is the default name for your remote, and 'main' is your default branch.",
                command: None,
                art_frames: &[
                    concat!(
                        "  LOCAL                REMOTE              \n",
                        "  main                 origin/main         \n",
                        "    ●──●──●            (empty)             \n",
                        "   c1 c2 c3                                \n",
                        "            ↑                              \n",
                        "         HEAD                              "
                    ),
                    concat!(
                        "  LOCAL                REMOTE              \n",
                        "  main                 origin/main         \n",
                        "    ●──●──●            ●──●──●             \n",
                        "   c1 c2 c3            c1 c2 c3            \n",
                        "            ↑                              \n",
                        "         HEAD                              "
                    ),
                ],
                result_frames: &[],
            },
            LessonStep {
                title: "Run the command",
                text: "This uploads your local commits to the remote, backing them up and sharing them with the team.",
                command: Some("git push origin main"),
                art_frames: &[
                    concat!(
                        "  LOCAL                REMOTE              \n",
                        "  main                 origin/main         \n",
                        "    ●──●──●            (empty)             \n",
                        "   c1 c2 c3                                \n",
                        "            ↑                              \n",
                        "         HEAD                              \n",
                        "  $ git push ...       "
                    ),
                ],
                result_frames: &[
                    concat!(
                        "  LOCAL                REMOTE              \n",
                        "  main                 origin/main         \n",
                        "    ●──●──●            ●──●──●             \n",
                        "   c1 c2 c3            c1 c2 c3            \n",
                        "            ↑                              \n",
                        "         HEAD                              \n",
                        "  ✓ Pushed 3 commits   "
                    ),
                    concat!(
                        "  LOCAL                REMOTE              \n",
                        "  main                 origin/main         \n",
                        "    ●──●──●            ●──●──●             \n",
                        "   c1 c2 c3            c1 c2 c3            \n",
                        "            ↑            ↑                 \n",
                        "         HEAD         HEAD                 \n",
                        "  ✓ In sync!           "
                    ),
                ],
            },
        ],
    }
}

fn lesson_branch() -> Lesson {
    Lesson {
        title: "The Branching Path",
        tagline: "Work in parallel without breaking the main line.",
        steps: &[
            LessonStep {
                title: "What is a branch?",
                text: "A branch is an independent line of development. It lets you experiment, build features, or fix bugs without affecting the main codebase. 'main' is the default branch. When you create a branch, Git makes a new pointer to the current commit.",
                command: None,
                art_frames: &[
                    concat!(
                        "                       \n",
                        "  main                 \n",
                        "    ●──●──●            \n",
                        "   c1 c2  c3           \n",
                        "            ↑          \n",
                        "         HEAD          \n",
                        "                       \n",
                        "  One timeline.        "
                    ),
                    concat!(
                        "                       \n",
                        "  main                 \n",
                        "    ●──●──●──●         \n",
                        "   c1 c2  c3  c4       \n",
                        "               ↑       \n",
                        "            HEAD       \n",
                        "                       \n",
                        "  Still one timeline.  "
                    ),
                    concat!(
                        "                       \n",
                        "  main                 \n",
                        "    ●──●──●            \n",
                        "   c1 c2  c3           \n",
                        "            ↑          \n",
                        "         HEAD          \n",
                        "                       \n",
                        "  Imagine two paths... "
                    ),
                ],
                result_frames: &[],
            },
            LessonStep {
                title: "Run the command",
                text: "This creates a new branch called 'feature-login' that points to the current commit. HEAD stays on main until you switch branches.",
                command: Some("git branch feature-login"),
                art_frames: &[
                    concat!(
                        "                       \n",
                        "  main                 \n",
                        "    ●──●──●            \n",
                        "   c1 c2  c3           \n",
                        "            ↑          \n",
                        "         HEAD          \n",
                        "                       \n",
                        "  $ git branch ...     "
                    ),
                ],
                result_frames: &[
                    concat!(
                        "                       \n",
                        "  main                 \n",
                        "    ●──●──●            \n",
                        "   c1 c2  c3           \n",
                        "            ↑          \n",
                        "         HEAD          \n",
                        "                       \n",
                        "  ✓ Branch created     "
                    ),
                    concat!(
                        "  main      feature-login\n",
                        "    ●──●──●              \n",
                        "   c1 c2  c3             \n",
                        "            ↑            \n",
                        "         HEAD            \n",
                        "                         \n",
                        "  Both point to c3       "
                    ),
                ],
            },
        ],
    }
}


fn lesson_switch() -> Lesson {
    Lesson {
        title: "The Great Switch",
        tagline: "Creating a branch is only half the journey.",
        steps: &[
            LessonStep {
                title: "What is git switch?",
                text: "Creating a branch doesn't move you onto it. 'git switch' moves HEAD to the target branch so your next commits happen there. It's the modern, clearer replacement for 'git checkout'.",
                command: None,
                art_frames: &[
                    concat!(
                        "  main      feature-login\n",
                        "    ●──●──●              \n",
                        "   c1 c2  c3             \n",
                        "            ↑            \n",
                        "         HEAD            \n",
                        "                         \n",
                        "  You're on main.        "
                    ),
                    concat!(
                        "  main      feature-login\n",
                        "    ●──●──●              \n",
                        "   c1 c2  c3             \n",
                        "            ↑            \n",
                        "         HEAD            \n",
                        "                         \n",
                        "  Want to work there?    "
                    ),
                ],
                result_frames: &[],
            },
            LessonStep {
                title: "Run the command",
                text: "This moves HEAD onto the feature-login branch. Any new commits you make now will extend that branch, not main.",
                command: Some("git switch feature-login"),
                art_frames: &[
                    concat!(
                        "  main      feature-login\n",
                        "    ●──●──●              \n",
                        "   c1 c2  c3             \n",
                        "            ↑            \n",
                        "         HEAD            \n",
                        "                         \n",
                        "  $ git switch ...       "
                    ),
                ],
                result_frames: &[
                    concat!(
                        "  main      feature-login\n",
                        "    ●──●──●              \n",
                        "   c1 c2  c3             \n",
                        "            ↑            \n",
                        "                      HEAD\n",
                        "                         \n",
                        "  ✓ Switched             "
                    ),
                    concat!(
                        "  main      feature-login\n",
                        "    ●──●──●──●           \n",
                        "   c1 c2  c3  c4         \n",
                        "               ↑         \n",
                        "                      HEAD\n",
                        "                         \n",
                        "  Commits go here now    "
                    ),
                ],
            },
        ],
    }
}

fn lesson_merge() -> Lesson {
    Lesson {
        title: "The Merge",
        tagline: "Bring separate paths back together.",
        steps: &[
            LessonStep {
                title: "What is git merge?",
                text: "When a feature branch is ready, you merge it back into main. Git creates a merge commit that has two parents — one from each branch — preserving the full history of both timelines.",
                command: None,
                art_frames: &[
                    concat!(
                        "  main                   \n",
                        "    ●──●──●              \n",
                        "   c1 c2  c3             \n",
                        "            ↑            \n",
                        "         HEAD            \n",
                        "              /           \n",
                        "               ●──●      \n",
                        "              c4  c5     \n",
                        "  feature-login          "
                    ),
                    concat!(
                        "  main                   \n",
                        "    ●──●──●              \n",
                        "   c1 c2  c3             \n",
                        "            ↑            \n",
                        "         HEAD            \n",
                        "              /           \n",
                        "               ●──●      \n",
                        "              c4  c5     \n",
                        "  Two timelines.         "
                    ),
                    concat!(
                        "  main                   \n",
                        "    ●──●──●──●           \n",
                        "   c1 c2  c3  m1         \n",
                        "            ↑ /           \n",
                        "         HEAD  ●──●      \n",
                        "              c4  c5     \n",
                        "  United.                "
                    ),
                ],
                result_frames: &[],
            },
            LessonStep {
                title: "Run the command",
                text: "This merges feature-login into the current branch (main). The merge commit m1 now has two parents: c3 and c5.",
                command: Some("git merge feature-login"),
                art_frames: &[
                    concat!(
                        "  main                   \n",
                        "    ●──●──●              \n",
                        "   c1 c2  c3             \n",
                        "            ↑            \n",
                        "         HEAD            \n",
                        "              /           \n",
                        "               ●──●      \n",
                        "              c4  c5     \n",
                        "  $ git merge ...        "
                    ),
                ],
                result_frames: &[
                    concat!(
                        "  main                   \n",
                        "    ●──●──●──●           \n",
                        "   c1 c2  c3  m1         \n",
                        "               ↑         \n",
                        "            HEAD         \n",
                        "            ↑ /           \n",
                        "             ●──●        \n",
                        "            c4  c5       \n",
                        "  ✓ Merged               "
                    ),
                    concat!(
                        "  main                   \n",
                        "    ●──●──●──●           \n",
                        "   c1 c2  c3  m1         \n",
                        "               ↑         \n",
                        "            HEAD         \n",
                        "            ↑ /           \n",
                        "             ●──●        \n",
                        "            c4  c5       \n",
                        "  History preserved.     "
                    ),
                ],
            },
        ],
    }
}

fn lesson_status() -> Lesson {
    Lesson {
        title: "The Status Check",
        tagline: "Know what's happening before you act.",
        steps: &[
            LessonStep {
                title: "What is git status?",
                text: "'git status' tells you exactly what's going on in your repository: which files are modified, which are staged, which are untracked, and which branch you're on. It's the first command to run when you're unsure.",
                command: None,
                art_frames: &[
                    concat!(
                        "  📁 project/            \n",
                        "  📄 index.html  [M]    \n",
                        "  📄 styles.css  [M]    \n",
                        "  🗑 .DS_Store   [?]    \n",
                        "  📄 README.md   [A]    \n",
                        "                        \n",
                        "  What's going on?!     "
                    ),
                    concat!(
                        "  On branch main         \n",
                        "  Changes not staged:    \n",
                        "    modified: index.html\n",
                        "    modified: styles.css\n",
                        "  Untracked files:       \n",
                        "    .DS_Store            \n",
                        "  Clarity.               "
                    ),
                ],
                result_frames: &[],
            },
            LessonStep {
                title: "Run the command",
                text: "This prints the current repository state: branch name, staged changes, unstaged changes, and untracked files.",
                command: Some("git status"),
                art_frames: &[
                    concat!(
                        "  📁 project/            \n",
                        "  📄 index.html  [M]    \n",
                        "  📄 styles.css  [M]    \n",
                        "  🗑 .DS_Store   [?]    \n",
                        "                        \n",
                        "  $ git status          "
                    ),
                ],
                result_frames: &[
                    concat!(
                        "  On branch main         \n",
                        "  Changes not staged:    \n",
                        "    modified: index.html\n",
                        "    modified: styles.css\n",
                        "  Untracked files:       \n",
                        "    .DS_Store            \n",
                        "  ✓ Now you know        "
                    ),
                    concat!(
                        "  On branch main         \n",
                        "  Your branch is clean  \n",
                        "                        \n",
                        "  Nothing to commit     \n",
                        "  working tree clean    \n",
                        "                        \n",
                        "  Peace of mind.        "
                    ),
                ],
            },
        ],
    }
}

fn lesson_log() -> Lesson {
    Lesson {
        title: "The History Book",
        tagline: "Every commit leaves a fingerprint.",
        steps: &[
            LessonStep {
                title: "What is git log?",
                text: "'git log' shows the commit history of your repository. Each entry displays the commit hash, author, date, and message. '--oneline' compresses each commit to a single line for quick scanning.",
                command: None,
                art_frames: &[
                    concat!(
                        "  main                   \n",
                        "    ●──●──●──●           \n",
                        "   c1 c2  c3  c4         \n",
                        "               ↑         \n",
                        "            HEAD         \n",
                        "                        \n",
                        "  What happened here?    "
                    ),
                    concat!(
                        "  a1b2c3d Add navbar    \n",
                        "  e4f5g6h Fix CSS bug   \n",
                        "  i7j8k9l Init repo     \n",
                        "                        \n",
                        "  A story in commits.    "
                    ),
                ],
                result_frames: &[],
            },
            LessonStep {
                title: "Run the command",
                text: "This displays the commit history in a compact one-line-per-commit format, newest first.",
                command: Some("git log --oneline"),
                art_frames: &[
                    concat!(
                        "  main                   \n",
                        "    ●──●──●──●           \n",
                        "   c1 c2  c3  c4         \n",
                        "               ↑         \n",
                        "            HEAD         \n",
                        "                        \n",
                        "  $ git log --oneline   "
                    ),
                ],
                result_frames: &[
                    concat!(
                        "  a1b2c3d Add navbar    \n",
                        "  e4f5g6h Fix CSS bug   \n",
                        "  i7j8k9l Init repo     \n",
                        "                        \n",
                        "  ✓ History revealed    "
                    ),
                    concat!(
                        "  a1b2c3d Add navbar    \n",
                        "  e4f5g6h Fix CSS bug   \n",
                        "  i7j8k9l Init repo     \n",
                        "                        \n",
                        "  3 commits, 3 stories.  "
                    ),
                ],
            },
        ],
    }
}

fn lesson_pull() -> Lesson {
    Lesson {
        title: "The Pull",
        tagline: "Stay in sync with the team.",
        steps: &[
            LessonStep {
                title: "What is git pull?",
                text: "'git pull' downloads new commits from the remote and merges them into your current branch. It's essentially 'git fetch' followed by 'git merge'. Use it to stay up to date before you start working.",
                command: None,
                art_frames: &[
                    concat!(
                        "  LOCAL                REMOTE              \n",
                        "  main                 origin/main         \n",
                        "    ●──●──●            ●──●──●──●         \n",
                        "   c1 c2 c3            c1 c2 c3 c4        \n",
                        "            ↑                              \n",
                        "         HEAD                              \n",
                        "  Remote is ahead.                        "
                    ),
                    concat!(
                        "  LOCAL                REMOTE              \n",
                        "  main                 origin/main         \n",
                        "    ●──●──●            ●──●──●──●         \n",
                        "   c1 c2 c3            c1 c2 c3 c4        \n",
                        "            ↑                    ↑         \n",
                        "         HEAD                 HEAD         \n",
                        "  Time to catch up.                       "
                    ),
                ],
                result_frames: &[],
            },
            LessonStep {
                title: "Run the command",
                text: "This fetches the latest commits from origin/main and merges them into your local main. Your local branch is now up to date.",
                command: Some("git pull origin main"),
                art_frames: &[
                    concat!(
                        "  LOCAL                REMOTE              \n",
                        "  main                 origin/main         \n",
                        "    ●──●──●            ●──●──●──●         \n",
                        "   c1 c2 c3            c1 c2 c3 c4        \n",
                        "            ↑                              \n",
                        "         HEAD                              \n",
                        "  $ git pull ...        "
                    ),
                ],
                result_frames: &[
                    concat!(
                        "  LOCAL                REMOTE              \n",
                        "  main                 origin/main         \n",
                        "    ●──●──●──●          ●──●──●──●         \n",
                        "   c1 c2 c3  c4         c1 c2 c3 c4        \n",
                        "               ↑                  ↑         \n",
                        "            HEAD               HEAD         \n",
                        "  ✓ Caught up             "
                    ),
                    concat!(
                        "  LOCAL                REMOTE              \n",
                        "  main                 origin/main         \n",
                        "    ●──●──●──●          ●──●──●──●         \n",
                        "   c1 c2 c3  c4         c1 c2 c3 c4        \n",
                        "               ↑                  ↑         \n",
                        "            HEAD               HEAD         \n",
                        "  In sync.               "
                    ),
                ],
            },
        ],
    }
}
