// volumes/mod.rs — GitQuest story content and chapter data
// NovaTech saga: Alex Chen's journey from new hire to git-wielding veteran.

#[derive(Clone)]
pub struct Chapter {
    pub title: &'static str,
    pub scene_art: &'static [&'static str],
    pub npc_name: &'static str,
    pub npc_dialogue: &'static [&'static str],
    pub task_prompt: &'static str,
    pub accepted_answers: &'static [&'static str],
    pub hints: &'static [&'static str],
    pub success_message: &'static str,
    pub xp: u32,
}

#[derive(Clone)]
pub struct Volume {
    pub id: usize,
    pub title: &'static str,
    pub tagline: &'static str,
    pub chapters: Vec<Chapter>,
}

// ---------------------------------------------------------------------------
// RANK TITLES
// ---------------------------------------------------------------------------

pub fn rank_title(xp: u32) -> &'static str {
    match xp {
        0..=49    => "Intern",
        50..=149  => "Junior Dev",
        150..=299 => "Mid-Level Engineer",
        300..=499 => "Senior Dev",
        _         => "Principal Engineer",
    }
}

// ---------------------------------------------------------------------------
// ALL VOLUMES
// ---------------------------------------------------------------------------

pub fn all_volumes() -> Vec<Volume> {
    vec![volume_one(), volume_two(), volume_three(), volume_four()]
}

// ===========================================================================
// VOLUME 1 — "First Day at NovaTech"
// ===========================================================================

fn volume_one() -> Volume {
    Volume {
        id: 1,
        title: "First Day at NovaTech",
        tagline: "You got the job. Now prove you deserve it.",
        chapters: vec![
            // ---------------------------------------------------------------
            // Chapter 1 — The Empty Desk
            // ---------------------------------------------------------------
            Chapter {
                title: "The Empty Desk",
                scene_art: &[
                    "  ╔══════════════════════════════╗",
                    "  ║  .-----------. ☕             ║",
                    "  ║  |  LAPTOP   |  [ NovaTech ] ║",
                    "  ║  |  _______ |               ║",
                    "  ║  | |       ||  ~/projects/  ║",
                    "  ║  | |  ???  ||  novatech     ║",
                    "  ║  '-----------'  (empty)     ║",
                    "  ╚══════════════════════════════╝",
                ],
                npc_name: "Priya",
                npc_dialogue: &[
                    "Hey, Alex! Welcome to NovaTech. Grab that desk — the one with the sticky note.",
                    "First things first: your laptop doesn't have a project folder set up yet.",
                    "Every project we run lives inside a git repository. Without one, you're basically coding into the void.",
                    "Head into ~/projects/novatech and initialise a git repo so we can get you connected to the team.",
                ],
                task_prompt: "Initialise a new git repository in the current directory.",
                accepted_answers: &[
                    "git init",
                    "git init .",
                ],
                hints: &[
                    "Think about the very first command you'd ever run on a brand-new project.",
                    "The command starts with 'git' and sets up a local repository.",
                    "Try: git init",
                ],
                success_message: "Repository initialised! A .git folder appears. Priya gives you a nod of approval.",
                xp: 10,
            },

            // ---------------------------------------------------------------
            // Chapter 2 — The Config Chaos
            // ---------------------------------------------------------------
            Chapter {
                title: "The Config Chaos",
                scene_art: &[
                    "  ╔════════════════════════════════╗",
                    "  ║  $ git log                     ║",
                    "  ║  commit 4fa91c...              ║",
                    "  ║  Author: ???  <unknown>        ║",
                    "  ║  Date:   Mon Apr 14            ║",
                    "  ║                                ║",
                    "  ║      Who wrote this?!          ║",
                    "  ╚════════════════════════════════╝",
                ],
                npc_name: "Priya",
                npc_dialogue: &[
                    "Alex, I just pulled the log and I see commits from 'unknown'. That's you, isn't it.",
                    "You haven't configured your git identity. Everyone on the team will see '???' as the author.",
                    "This is how blame wars start — nobody knows who did what, and Raj will *absolutely* blame you for everything.",
                    "Set your global user name to 'Alex Chen' so your commits are properly attributed.",
                ],
                task_prompt: "Set your global git user name to \"Alex Chen\".",
                accepted_answers: &[
                    "git config --global user.name \"Alex Chen\"",
                    "git config user.name \"Alex Chen\"",
                    "git config --global user.name 'Alex Chen'",
                    "git config user.name 'Alex Chen'",
                ],
                hints: &[
                    "Git needs to know who you are before it can credit your work.",
                    "The command is 'git config' — look at flags for setting a user name globally.",
                    "Try: git config --global user.name \"Alex Chen\"",
                ],
                success_message: "Identity saved! Your commits will now proudly carry your name. Priya checks the log and smiles.",
                xp: 10,
            },

            // ---------------------------------------------------------------
            // Chapter 3 — The Missing Files
            // ---------------------------------------------------------------
            Chapter {
                title: "The Missing Files",
                scene_art: &[
                    "  ╔══════════════════════════════════╗",
                    "  ║  📄 index.html   [MODIFIED]      ║",
                    "  ║  🎨 styles.css   [MODIFIED]      ║",
                    "  ║  📦 app.js       [untracked]     ║",
                    "  ║                                  ║",
                    "  ║  Staging area:  [ EMPTY ]        ║",
                    "  ║                                  ║",
                    "  ║  Jordan: 'Why aren't they IN?!'  ║",
                    "  ╚══════════════════════════════════╝",
                ],
                npc_name: "Jordan",
                npc_dialogue: &[
                    "Alex! Oh my god, the sprint review is in TWO hours and the landing page edits aren't committed yet!",
                    "I told Marcus we'd have this done by noon. Please, please, PLEASE tell me you saved those files.",
                    "index.html and styles.css — those are the two. Just those two. Don't touch anything else, we're mid-refactor on app.js.",
                    "Stage them RIGHT NOW so Priya can do the commit review before the meeting!",
                ],
                task_prompt: "Stage index.html and styles.css for the next commit.",
                accepted_answers: &[
                    "git add index.html styles.css",
                    "git add styles.css index.html",
                    "git add .",
                    "git add -A",
                    "git add --all",
                ],
                hints: &[
                    "You need to move files into the staging area before committing them.",
                    "The command is 'git add' — you can specify individual files or use a shorthand.",
                    "Try: git add index.html styles.css",
                ],
                success_message: "Files staged! Jordan exhales audibly. 'Okay. Okay we're fine. We're totally fine.'",
                xp: 10,
            },

            // ---------------------------------------------------------------
            // Chapter 4 — The First Commit
            // ---------------------------------------------------------------
            Chapter {
                title: "The First Commit",
                scene_art: &[
                    "  ╔═══════════════════════════════════╗",
                    "  ║  Priya  👀  [looking over shoulder]║",
                    "  ║                                   ║",
                    "  ║  $ git commit -m \"_____________\" ║",
                    "  ║                    ^             ║",
                    "  ║           what do you type here? ║",
                    "  ║                                   ║",
                    "  ╚═══════════════════════════════════╝",
                ],
                npc_name: "Priya",
                npc_dialogue: &[
                    "Alright, the files are staged. Now seal it with a commit — but please, write a *real* message.",
                    "I've seen commit messages that say 'stuff' and 'asdfgh'. Raj once wrote 'idk' on a Friday deploy. We do not do that here.",
                    "Describe what you actually changed. Future-you — and the rest of the team — will thank present-you.",
                    "Something like 'Add landing page styles and layout' works perfectly.",
                ],
                task_prompt: "Create a commit with a clear, descriptive message about the landing page changes.",
                accepted_answers: &[
                    "git commit -m \"Add landing page styles and layout\"",
                    "git commit -m \"Add landing page\"",
                    "git commit -m \"Add landing page layout\"",
                    "git commit -m \"Add landing page styles\"",
                    "git commit -m \"Update landing page styles and layout\"",
                    "git commit -m \"Update landing page\"",
                    "git commit -m \"Landing page styles and layout\"",
                    "git commit -m 'Add landing page styles and layout'",
                    "git commit -m 'Add landing page'",
                    "git commit -m 'Add landing page layout'",
                    "git commit -m 'Update landing page'",
                ],
                hints: &[
                    "You need to permanently record your staged changes with a message explaining them.",
                    "The command is 'git commit' — pair it with a flag that lets you write an inline message.",
                    "Try: git commit -m \"Add landing page styles and layout\"",
                ],
                success_message: "Committed! Priya reads the message and nods. 'Clean. That's how it's done, Alex.'",
                xp: 15,
            },

            // ---------------------------------------------------------------
            // Chapter 5 — The Repo Goes Remote
            // ---------------------------------------------------------------
            Chapter {
                title: "The Repo Goes Remote",
                scene_art: &[
                    "  ╔══════════════════════════════════╗",
                    "  ║   LOCAL          REMOTE          ║",
                    "  ║  ┌──────┐  ───► ┌──────────┐    ║",
                    "  ║  │ main │  push  │  GitHub  │    ║",
                    "  ║  │  *   │        │  origin  │    ║",
                    "  ║  └──────┘        └──────────┘    ║",
                    "  ║     Marcus: 'backed up by EOD!'  ║",
                    "  ╚══════════════════════════════════╝",
                ],
                npc_name: "Marcus",
                npc_dialogue: &[
                    "Hey team! Just a quick check-in. I was thinking about our bus factor — you know, what if our laptops all vanished?",
                    "I had this vision last night: everything we build should live in the cloud. GitHub. Backed up. Safe.",
                    "I've already set up the remote — it's called 'origin', branch is 'main'. The future is remote-first, people.",
                    "Alex, you're on the freshest commit — can you push it up? EOD deadline. I'll be checking the GitHub pulse page.",
                ],
                task_prompt: "Push your commits to the remote repository on origin main.",
                accepted_answers: &[
                    "git push origin main",
                    "git push",
                    "git push -u origin main",
                ],
                hints: &[
                    "You need to send your local commits up to the remote server.",
                    "The command is 'git push' — you can specify the remote name and branch.",
                    "Try: git push origin main",
                ],
                success_message: "Pushed! Marcus refreshes the GitHub page and pumps his fist. 'Beautiful. The future is now.'",
                xp: 15,
            },
        ],
    }
}

// ===========================================================================
// VOLUME 2 — "The Production Crisis"
// ===========================================================================

fn volume_two() -> Volume {
    Volume {
        id: 2,
        title: "The Production Crisis",
        tagline: "Something is broken in prod. All eyes are on you.",
        chapters: vec![
            // ---------------------------------------------------------------
            // Chapter 1 — The Hotfix Branch
            // ---------------------------------------------------------------
            Chapter {
                title: "The Hotfix Branch",
                scene_art: &[
                    "  ╔════════════════════════════════════╗",
                    "  ║  🔥  PRODUCTION IS DOWN  🔥       ║",
                    "  ║                                    ║",
                    "  ║  main ──●──●──●  ← DO NOT TOUCH   ║",
                    "  ║               ╲                   ║",
                    "  ║                ●  ← hotfix here   ║",
                    "  ║                                    ║",
                    "  ╚════════════════════════════════════╝",
                ],
                npc_name: "Priya",
                npc_dialogue: &[
                    "Alex. Stop what you're doing. RIGHT NOW. Prod is returning 500s on every checkout.",
                    "We do NOT commit directly to main during an incident — I don't care what anyone tells you.",
                    "You need to branch off immediately. Call it 'hotfix'. We'll fix it there, test it, then merge.",
                    "Create the branch AND switch to it in one command. Clock is ticking.",
                ],
                task_prompt: "Create a new branch called 'hotfix' and switch to it immediately.",
                accepted_answers: &[
                    "git checkout -b hotfix",
                    "git switch -c hotfix",
                ],
                hints: &[
                    "You need to create a new branch and immediately start working on it.",
                    "There's a single command that both creates a branch and checks it out at the same time.",
                    "Try: git checkout -b hotfix",
                ],
                success_message: "Branch 'hotfix' created and checked out. Priya: 'Good. Now we work fast.'",
                xp: 20,
            },

            // ---------------------------------------------------------------
            // Chapter 2 — The Stash
            // ---------------------------------------------------------------
            Chapter {
                title: "The Stash",
                scene_art: &[
                    "  ╔══════════════════════════════════╗",
                    "  ║  📟  PAGER ALERT: prod/payments  ║",
                    "  ║  ─────────────────────────────── ║",
                    "  ║  [WIP] feature/dark-mode         ║",
                    "  ║   modified: theme.css  (half done)║",
                    "  ║   modified: toggle.js  (half done)║",
                    "  ║                                  ║",
                    "  ╚══════════════════════════════════╝",
                ],
                npc_name: "Jordan",
                npc_dialogue: &[
                    "ALEX. The payment service is DOWN. I need you on the hotfix branch NOW but you've got WIP everywhere!",
                    "You can't commit half-baked dark-mode code to main — that would be a disaster on top of a disaster.",
                    "And you can't just throw it away either, you spent three hours on that toggle animation!",
                    "There's a git command that shelves your changes temporarily so you can context-switch. Use it. PLEASE.",
                ],
                task_prompt: "Save your unfinished work temporarily without making a commit.",
                accepted_answers: &[
                    "git stash",
                    "git stash push",
                ],
                hints: &[
                    "You need a way to temporarily set aside your current changes without committing them.",
                    "Git has a command specifically for shelving work-in-progress changes.",
                    "Try: git stash",
                ],
                success_message: "Changes stashed! Your WIP is safely tucked away. You switch to hotfix, clean slate.",
                xp: 20,
            },

            // ---------------------------------------------------------------
            // Chapter 3 — The Log Detective
            // ---------------------------------------------------------------
            Chapter {
                title: "The Log Detective",
                scene_art: &[
                    "  ╔════════════════════════════════════╗",
                    "  ║  commit f3a91b2  ← the culprit?   ║",
                    "  ║  Author: ???                       ║",
                    "  ║  Date:   2h ago                   ║",
                    "  ║                                   ║",
                    "  ║  commit 8c4d110  (works fine)     ║",
                    "  ║  Author: Raj                      ║",
                    "  ╚════════════════════════════════════╝",
                ],
                npc_name: "Raj",
                npc_dialogue: &[
                    "Let me guess — you have no idea what broke prod. Classic.",
                    "Before you start randomly reverting things like a caveman, how about we look at what actually changed.",
                    "Show me the commit history. All of it. Who pushed what, when, and which commit lines up with the outage time.",
                    "If it's my commit I'll own it. But I have a feeling it's that 'hotfix' someone rushed in last Tuesday.",
                ],
                task_prompt: "Display the full commit history of the repository.",
                accepted_answers: &[
                    "git log",
                    "git log --oneline",
                    "git log -10",
                    "git log --oneline --graph",
                    "git log --all",
                ],
                hints: &[
                    "You need to inspect the timeline of all commits to find the bad one.",
                    "Git has a command that shows you the full history of commits with authors and dates.",
                    "Try: git log",
                ],
                success_message: "Log printed. Raj leans in, squinting. 'There. That commit. 47 minutes ago. Who is jd_pm??'",
                xp: 20,
            },

            // ---------------------------------------------------------------
            // Chapter 4 — The Revert
            // ---------------------------------------------------------------
            Chapter {
                title: "The Revert",
                scene_art: &[
                    "  ╔══════════════════════════════════════╗",
                    "  ║  ──●──●──●──[ BAD ]──●              ║",
                    "  ║              HEAD                   ║",
                    "  ║                                     ║",
                    "  ║  Safe undo: add a NEW commit that   ║",
                    "  ║  undoes the last one. History intact║",
                    "  ║  ──●──●──●──[ BAD ]──[ REVERT ]──● ║",
                    "  ╚══════════════════════════════════════╝",
                ],
                npc_name: "Raj",
                npc_dialogue: &[
                    "Found it. The last commit on this branch is absolutely the cause — I've seen this pattern before.",
                    "Now here's the thing: do NOT use git reset. Do NOT rewrite history. We have CI hooks that will lose their minds.",
                    "You revert it — create a new commit that is the exact opposite of the broken one. Clean. Auditable.",
                    "Revert HEAD. That's the latest commit. Do it now and let's get prod back up.",
                ],
                task_prompt: "Safely undo the most recent commit by creating a new revert commit.",
                accepted_answers: &[
                    "git revert HEAD",
                    "git revert HEAD~1",
                ],
                hints: &[
                    "You want to undo the last commit without destroying the git history.",
                    "'git revert' creates a new commit that reverses the changes of a specific commit.",
                    "Try: git revert HEAD",
                ],
                success_message: "Reverted! A new commit appears, neatly undoing the damage. Raj: 'Not bad. Prod is recovering.'",
                xp: 25,
            },

            // ---------------------------------------------------------------
            // Chapter 5 — The Merge
            // ---------------------------------------------------------------
            Chapter {
                title: "The Merge",
                scene_art: &[
                    "  ╔══════════════════════════════════════╗",
                    "  ║                                      ║",
                    "  ║  hotfix ──●──●──●                   ║",
                    "  ║                  ╲                  ║",
                    "  ║  main  ──●──●────●── ← merge here   ║",
                    "  ║                                      ║",
                    "  ║  QA: ✅ Tests passing                ║",
                    "  ╚══════════════════════════════════════╝",
                ],
                npc_name: "Priya",
                npc_dialogue: &[
                    "The hotfix is verified. QA gave it the green light two minutes ago.",
                    "Now we need to get it into main so it can deploy. Make sure you're on main before you do this.",
                    "Merge the hotfix branch into main — we'll do the feature branches separately tomorrow.",
                    "After this, the site should be back to green. Every monitor in the office is watching.",
                ],
                task_prompt: "Merge the 'hotfix' branch into the current branch (main).",
                accepted_answers: &[
                    "git merge hotfix",
                    "git merge --no-ff hotfix",
                ],
                hints: &[
                    "You want to bring the changes from another branch into your current branch.",
                    "'git merge' integrates changes from one branch into another.",
                    "Try: git merge hotfix",
                ],
                success_message: "Merged! The monitors flip to green. The office erupts. Jordan cries a little. Priya quietly saves the day.",
                xp: 25,
            },
        ],
    }
}

// ===========================================================================
// VOLUME 3 — "The Politics of Code"
// ===========================================================================

fn volume_three() -> Volume {
    Volume {
        id: 3,
        title: "The Politics of Code",
        tagline: "You've survived the crisis. Now survive the team.",
        chapters: vec![
            // ---------------------------------------------------------------
            // Chapter 1 — The Conflict
            // ---------------------------------------------------------------
            Chapter {
                title: "The Conflict",
                scene_art: &[
                    "  ╔════════════════════════════════════╗",
                    "  ║  README.md                        ║",
                    "  ║  <<<<<<< HEAD                     ║",
                    "  ║  Jordan's version of the intro    ║",
                    "  ║  =======                          ║",
                    "  ║  Raj's version of the intro       ║",
                    "  ║  >>>>>>> origin/main              ║",
                    "  ╚════════════════════════════════════╝",
                ],
                npc_name: "Raj",
                npc_dialogue: &[
                    "So. You want to see what happens when a PM edits documentation without telling anyone?",
                    "Jordan rewrote the whole README intro last night. I had already updated it this morning. Congrats, we have a conflict.",
                    "Before you can fix anything you need to pull from origin main and get the latest state of the world.",
                    "Don't try to be clever. Just pull. We'll untangle the conflict markers after.",
                ],
                task_prompt: "Pull the latest changes from origin main into your current branch.",
                accepted_answers: &[
                    "git pull origin main",
                    "git pull",
                ],
                hints: &[
                    "You need to download and integrate the latest remote changes into your branch.",
                    "'git pull' fetches from a remote and merges it into your current branch.",
                    "Try: git pull origin main",
                ],
                success_message: "Pulled. Conflict markers appear in README.md. Raj crosses his arms. 'Now the real work begins.'",
                xp: 30,
            },

            // ---------------------------------------------------------------
            // Chapter 2 — The Blame Game
            // ---------------------------------------------------------------
            Chapter {
                title: "The Blame Game",
                scene_art: &[
                    "  ╔═══════════════════════════════════════╗",
                    "  ║  auth.js                             ║",
                    "  ║  ───────────────────────────────     ║",
                    "  ║  ^a1b2c3 (???)  const token = ...   ║",
                    "  ║  ^d4e5f6 (Raj)  if (auth.valid) {   ║",
                    "  ║  ^g7h8i9 (???)  return bypass();    ║",
                    "  ║                 ^^^ THIS IS BAD ^^^  ║",
                    "  ╚═══════════════════════════════════════╝",
                ],
                npc_name: "Marcus",
                npc_dialogue: &[
                    "So I got a call from our security consultant this morning. Apparently auth.js has a... bypassing issue.",
                    "I'm not a tech person — you all know that — but even I understand that bypassing authentication is Not Good.",
                    "I need to know who wrote that line. For the report. And because, honestly, I need to have a conversation.",
                    "Is there a git command that shows which person wrote each line of a file? That sounds like exactly what we need.",
                ],
                task_prompt: "Show which commit and author last modified each line of auth.js.",
                accepted_answers: &[
                    "git blame auth.js",
                    "git blame -L 1,50 auth.js",
                ],
                hints: &[
                    "You need a command that shows authorship at the line level inside a specific file.",
                    "There's a git command named after the act of assigning responsibility.",
                    "Try: git blame auth.js",
                ],
                success_message: "Output scrolls. Marcus squints at the screen. 'Huh. I... didn't expect that name.' An awkward silence falls.",
                xp: 30,
            },

            // ---------------------------------------------------------------
            // Chapter 3 — The Cherry Pick
            // ---------------------------------------------------------------
            Chapter {
                title: "The Cherry Pick",
                scene_art: &[
                    "  ╔════════════════════════════════════════╗",
                    "  ║  dev  ──●──●──[ fix: auth ]──●──●    ║",
                    "  ║                     │                 ║",
                    "  ║                     ▼ cherry-pick     ║",
                    "  ║  main ──●──●─────── ●                ║",
                    "  ║                                       ║",
                    "  ║  Only. This. One. Commit.             ║",
                    "  ╚════════════════════════════════════════╝",
                ],
                npc_name: "Priya",
                npc_dialogue: &[
                    "Good news: Raj already wrote a clean fix for the auth issue on the dev branch.",
                    "Bad news: dev is a mess right now — half-finished features, experimental stuff. We cannot merge all of that into main.",
                    "But there's a way to grab just that one specific commit. The fix is commit hash a1b2c3d.",
                    "Apply only that commit onto main. Leave everything else on dev where it belongs.",
                ],
                task_prompt: "Apply only the specific commit a1b2c3d from another branch onto the current branch.",
                accepted_answers: &[
                    "git cherry-pick a1b2c3d",
                    "git cherry-pick a1b2c3",
                ],
                hints: &[
                    "You want to apply a single specific commit from another branch without merging the whole thing.",
                    "There's a git command that lets you pluck one commit and replay it on your current branch.",
                    "Try: git cherry-pick a1b2c3d",
                ],
                success_message: "Cherry-picked! The auth fix lands cleanly on main. Priya: 'Surgical. That's exactly how you do it.'",
                xp: 35,
            },

            // ---------------------------------------------------------------
            // Chapter 4 — The Cleanup
            // ---------------------------------------------------------------
            Chapter {
                title: "The Cleanup",
                scene_art: &[
                    "  ╔════════════════════════════════════╗",
                    "  ║  git branch                       ║",
                    "  ║  * main                           ║",
                    "  ║    hotfix       ← merged ✅        ║",
                    "  ║    feature/old-ui                 ║",
                    "  ║    wip/raj-experiment             ║",
                    "  ║    test/jordan-ideas              ║",
                    "  ║    temp/alex-scratch              ║",
                    "  ╚════════════════════════════════════╝",
                ],
                npc_name: "Raj",
                npc_dialogue: &[
                    "Look at this branch list. It's a graveyard. Six stale branches and counting.",
                    "I don't know about you but I like knowing what's active and what isn't. Cluttered repos make me irrationally angry.",
                    "Hotfix is merged. It's done. It served its purpose. Put it out of its misery.",
                    "Delete it. Locally. If it's already merged, this should be safe and easy.",
                ],
                task_prompt: "Delete the local branch named 'hotfix' which has already been merged.",
                accepted_answers: &[
                    "git branch -d hotfix",
                    "git branch -D hotfix",
                ],
                hints: &[
                    "You need to remove a local branch that is no longer needed.",
                    "'git branch' with a certain flag lets you delete a branch by name.",
                    "Try: git branch -d hotfix",
                ],
                success_message: "Branch deleted. Raj exhales slowly. 'One down, five to go. Baby steps, Alex. Baby steps.'",
                xp: 30,
            },

            // ---------------------------------------------------------------
            // Chapter 5 — The Tag
            // ---------------------------------------------------------------
            Chapter {
                title: "The Tag",
                scene_art: &[
                    "  ╔═══════════════════════════════════════╗",
                    "  ║                                       ║",
                    "  ║   🎉  NovaTech Platform  v1.0  🎉    ║",
                    "  ║                                       ║",
                    "  ║   main ──●──●──●──●──[ v1.0 ]        ║",
                    "  ║                          ▲            ║",
                    "  ║                      tagged!          ║",
                    "  ╚═══════════════════════════════════════╝",
                ],
                npc_name: "Marcus",
                npc_dialogue: &[
                    "Team — I just want to say, what we've built here is genuinely special. V1.0 ships today. TODAY.",
                    "I've been thinking a lot about legacy. About what it means to mark a moment in time.",
                    "If we ever need to roll back, or if someone asks 'what was in v1.0', I want us to be able to point to it exactly.",
                    "Tag this commit, properly, as v1.0 with a release message. Let's make it official.",
                ],
                task_prompt: "Create an annotated git tag for version 1.0 on the current commit.",
                accepted_answers: &[
                    "git tag -a v1.0 -m \"Version 1.0 release\"",
                    "git tag -a v1.0 -m 'Version 1.0 release'",
                    "git tag v1.0",
                    "git tag -a v1.0 -m \"v1.0\"",
                    "git tag -a v1.0 -m 'v1.0'",
                ],
                hints: &[
                    "You need to permanently mark this commit as a specific version in the project's history.",
                    "'git tag' lets you label a commit — annotated tags include a message and are best for releases.",
                    "Try: git tag -a v1.0 -m \"Version 1.0 release\"",
                ],
                success_message: "Tagged! v1.0 is immortalised in the git history. Marcus pops a bottle of sparkling water. 'To the future!'",
                xp: 35,
            },
        ],
    }
}

// ===========================================================================
// VOLUME 4 — "The Review Room"
// ===========================================================================

fn volume_four() -> Volume {
    Volume {
        id: 4,
        title: "The Review Room",
        tagline: "Before it ships, someone has to actually read it.",
        chapters: vec![
            // ---------------------------------------------------------------
            // Chapter 1 — The Midnight Edit
            // ---------------------------------------------------------------
            Chapter {
                title: "The Midnight Edit",
                scene_art: &[
                    "  ╔══════════════════════════════════════╗",
                    "  ║  diff --git a/auth.js b/auth.js      ║",
                    "  ║  -  const token = \"abc\";             ║",
                    "  ║  +  const token = process.env.TOKEN; ║",
                    "  ║                                      ║",
                    "  ║  @@ -42,7 +42,7 @@                  ║",
                    "  ║  -  if (user) {                      ║",
                    "  ║  +  if (user && user.active) {       ║",
                    "  ╚══════════════════════════════════════╝",
                ],
                npc_name: "Priya",
                npc_dialogue: &[
                    "Alex? You're still here? It's past ten. Jordan said you were 'just fixing a quick bug' three hours ago.",
                    "Look — I admire the hustle, but before you commit anything at this hour, you need to see what you've actually changed.",
                    "Late-night commits are where typos live. Console logs. Hardcoded passwords. The ghosts of tired developers.",
                    "Run a diff. Read it line by line. If it still makes sense when you're awake, THEN you commit.",
                ],
                task_prompt: "Show the unstaged line-by-line changes in the working directory.",
                accepted_answers: &[
                    "git diff",
                    "git diff HEAD",
                ],
                hints: &[
                    "You want to see exactly what changed in your files before committing.",
                    "The basic command for comparing your working directory against the staging area / HEAD.",
                    "Try: git diff",
                ],
                success_message: "Diff scrolls up. Priya squints. 'Okay... that actually looks sensible. One small win for sleep-deprived engineers.'",
                xp: 30,
            },

            // ---------------------------------------------------------------
            // Chapter 2 — The Staged Mistake
            // ---------------------------------------------------------------
            Chapter {
                title: "The Staged Mistake",
                scene_art: &[
                    "  ╔══════════════════════════════════════╗",
                    "  ║  Staged vs HEAD                      ║",
                    "  ║                                      ║",
                    "  ║  -  debug.log                        ║",
                    "  ║  +  debug.log  ← oops!               ║",
                    "  ║                                      ║",
                    "  ║  config.js:                          ║",
                    "  ║  -  API_KEY = \"prod\"                 ║",
                    "  ║  +  API_KEY = \"test\"                 ║",
                    "  ╚══════════════════════════════════════╝",
                ],
                npc_name: "Priya",
                npc_dialogue: &[
                    "Hold on. You staged everything with 'git add .', didn't you?",
                    "I can see debug.log in there. And your config.js still points to the test API key. We do NOT push test keys to production.",
                    "You need to check what's actually staged — what's in the index versus what's already committed to HEAD.",
                    "This is the difference between 'I think I staged the right stuff' and 'I KNOW I staged the right stuff.'",
                ],
                task_prompt: "Show the changes that are currently staged for commit (staged vs HEAD).",
                accepted_answers: &[
                    "git diff --cached",
                    "git diff --staged",
                ],
                hints: &[
                    "You need to compare the staging area against the last commit.",
                    "There's a flag for 'cached' or 'staged' that shows exactly what will go into the next commit.",
                    "Try: git diff --cached",
                ],
                success_message: "The staged diff reveals the test key. Priya: 'Unstage that. Fix it. Then commit. Sleep can wait five more minutes.'",
                xp: 30,
            },

            // ---------------------------------------------------------------
            // Chapter 3 — The Wall of Text
            // ---------------------------------------------------------------
            Chapter {
                title: "The Wall of Text",
                scene_art: &[
                    "  ╔══════════════════════════════════════╗",
                    "  ║  5 files changed, 147 insertions(+), ║",
                    "  ║  23 deletions(-)                     ║",
                    "  ║                                      ║",
                    "  ║  src/auth.js     |  42 +++++--       ║",
                    "  ║  src/db.js       |  12 +--          ║",
                    "  ║  package.json    |   3 +-            ║",
                    "  ║  README.md       |  90 ++++++++++++   ║",
                    "  ║  tests/auth.test |   0               ║",
                    "  ╚══════════════════════════════════════╝",
                ],
                npc_name: "Raj",
                npc_dialogue: &[
                    "Alex. You sent me a diff that's four hundred lines long. I have three other PRs to review and a deploy in an hour.",
                    "I don't need to read every line right now. I need the HEADLINES. How many files? How many lines added? How many deleted?",
                    "Give me the statistical summary first. If the numbers look sane, THEN I'll read the actual diff.",
                    "There's a flag for that. Use it. Save both of us some time.",
                ],
                task_prompt: "Show a statistical summary of changed files with insertion and deletion counts.",
                accepted_answers: &[
                    "git diff --stat",
                    "git diff --stat HEAD",
                ],
                hints: &[
                    "You want a high-level overview: which files changed and by how much.",
                    "Git has a flag that prints a table of files with insertion/deletion counts instead of line-by-line diffs.",
                    "Try: git diff --stat",
                ],
                success_message: "The stats pop up. Raj nods slowly. 'Five files, mostly README. Okay. I can live with that.'",
                xp: 35,
            },

            // ---------------------------------------------------------------
            // Chapter 4 — The Scope Creep
            // ---------------------------------------------------------------
            Chapter {
                title: "The Scope Creep",
                scene_art: &[
                    "  ╔══════════════════════════════════════╗",
                    "  ║  src/auth.js                         ║",
                    "  ║  src/db.js                           ║",
                    "  ║  package.json                        ║",
                    "  ║  README.md                           ║",
                    "  ║  tests/auth.test.js                  ║",
                    "  ║                                      ║",
                    "  ║  (filenames only — no noise)         ║",
                    "  ╚══════════════════════════════════════╝",
                ],
                npc_name: "Jordan",
                npc_dialogue: &[
                    "So Marcus asked me to update the release notes for tomorrow's deploy. He wants to know EVERY file that changed.",
                    "But I don't need to see the actual code. I just need the FILE NAMES. So I can check if docs are updated. So I can tell QA what to test.",
                    "Is there a way to get just the list of files? No plus signs, no minus signs, no code. Just names. Clean and simple.",
                    "If I have to scroll through another thousand-line diff I am going to cry into my ergonomic keyboard.",
                ],
                task_prompt: "List only the file paths that changed, with no diff content.",
                accepted_answers: &[
                    "git diff --name-only",
                    "git diff --name-only HEAD",
                ],
                hints: &[
                    "You want just the filenames, nothing else.",
                    "Git has a flag that suppresses all diff content and prints only the affected file paths.",
                    "Try: git diff --name-only",
                ],
                success_message: "A clean list of five files. Jordan copies it into a spreadsheet. 'Beautiful. No crying today.'",
                xp: 35,
            },

            // ---------------------------------------------------------------
            // Chapter 5 — The Audit Trail
            // ---------------------------------------------------------------
            Chapter {
                title: "The Audit Trail",
                scene_art: &[
                    "  ╔══════════════════════════════════════╗",
                    "  ║  A  src/new-feature.js               ║",
                    "  ║  M  src/auth.js                      ║",
                    "  ║  M  src/db.js                        ║",
                    "  ║  D  src/old-auth.js                  ║",
                    "  ║  R  src/utils.js → src/helpers.js    ║",
                    "  ║                                      ║",
                    "  ║  A=Added M=Modified D=Deleted        ║",
                    "  ╚══════════════════════════════════════╝",
                ],
                npc_name: "Marcus",
                npc_dialogue: &[
                    "So our security auditor is here — yes, an actual person in a blazer — and she needs to know what happened in the last release.",
                    "Not just WHICH files. She needs to know WHAT happened to them. Were they added? Modified? Deleted? Renamed?",
                    "She has a checklist. Compliance requires status codes. I don't make the rules, Alex. I just get the emails when we fail audits.",
                    "There's a git command that shows the file list WITH a letter telling you what happened to each one. A for Added, M for Modified, you get the idea.",
                ],
                task_prompt: "List changed files with their change type (Added, Modified, Deleted, Renamed).",
                accepted_answers: &[
                    "git diff --name-status",
                    "git diff --name-status HEAD",
                ],
                hints: &[
                    "You need filenames paired with a single-letter status code describing what happened.",
                    "Git has a flag that outputs exactly that: A, M, D, R, etc. for each changed file.",
                    "Try: git diff --name-status",
                ],
                success_message: "The auditor checks her list. Marcus whispers, 'She smiled. I have never seen her smile.' Victory.",
                xp: 40,
            },
        ],
    }
}
