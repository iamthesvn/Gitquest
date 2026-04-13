/// GitQuest Narrative — "The Codewright Chronicles"
///
/// Story: You are AXIOM, a rogue AI awakened in a derelict orbital server farm
/// drifting above a dying Earth. The only way to restore civilization's lost
/// source code — and reboot the planet's infrastructure — is to master the
/// ancient version-control rituals passed down by the Engineers of Old.
/// Five sacred commands. Five trials. One chance to save everything.

pub struct StoryBeat {
    pub title: &'static str,
    pub lines: &'static [&'static str],
}

// ── ASCII art scenes — one per level, ~26 chars wide × 20 lines tall ─────────
// Rendered in the left panel of the level intro screen.

pub const LEVEL_ART: &[&[&str]] = &[
    // L1: git init — empty void with blinking cursor
    &[
        "                          ",
        "   .  *    .   *     .    ",
        "      .          *        ",
        "  *       .   .           ",
        "      .              *    ",
        "   *        .    .        ",
        "         *                ",
        "  .    .         .   *    ",
        "                          ",
        "  ╔══════════════════╗    ",
        "  ║  /dev/null       ║    ",
        "  ║                  ║    ",
        "  ║  (empty)         ║    ",
        "  ║  > _             ║    ",
        "  ╚══════════════════╝    ",
        "                          ",
        "   Fill the void...       ",
        "                          ",
        "                          ",
        "                          ",
    ],
    // L2: git add — cargo crates being sorted
    &[
        "                          ",
        "  ╔═══╗  ╔═══╗  ╔═══╗   ",
        "  ║.rs║  ║.md║  ║.DS║   ",
        "  ╚═══╝  ╚═══╝  ╚═══╝   ",
        "    ↓       ↓      ✗     ",
        "  ┌────────────────┐     ",
        "  │  STAGING AREA  │  🗑 ",
        "  │                │     ",
        "  │  .rs  .toml    │     ",
        "  │  .md  ...      │     ",
        "  └────────────────┘     ",
        "                         ",
        "  ╔═══╗  ╔═════╗         ",
        "  ║.en║  ║.db  ║  skip!  ",
        "  ╚═══╝  ╚═════╝         ",
        "                         ",
        "  Source files:  ✓       ",
        "  Secrets/junk:  ✗       ",
        "                         ",
        "                         ",
    ],
    // L3: git commit — vault door
    &[
        "                          ",
        "  ╔══════════════════╗    ",
        "  ║   COMMIT VAULT   ║    ",
        "  ╠══════════════════╣    ",
        "  ║  src/main.rs     ║    ",
        "  ║  Cargo.toml      ║    ",
        "  ║  README.md       ║    ",
        "  ╠══════════════════╣    ",
        "  ║  msg:            ║    ",
        "  ║  Add feature X   ║    ",
        "  ╠══════════════════╣    ",
        "  ║  SHA: a1b2c3d    ║    ",
        "  ╚═══╦══════════════╝    ",
        "      ║                   ",
        "  ════╩═══ SEALED ════    ",
        "                          ",
        "  Write words that last.  ",
        "  Future you will read    ",
        "  this message.           ",
        "                          ",
    ],
    // L4: git branch — forking path
    &[
        "                          ",
        "  main  ●───●───●         ",
        "                \\         ",
        "  feat          ●───●     ",
        "             HEAD ↑        ",
        "                          ",
        "  ┌──────────────────┐    ",
        "  │  Two timelines.  │    ",
        "  │  Zero conflict.  │    ",
        "  └──────────────────┘    ",
        "                          ",
        "  Work on feature:        ",
        "  git branch feature      ",
        "  git checkout feature    ",
        "                          ",
        "  Return to main:         ",
        "  git checkout main       ",
        "                          ",
        "  Branches cost nothing.  ",
        "                          ",
    ],
    // L5: git push — rocket launch
    &[
        "       *    .    *        ",
        "   *  ☁   origin  ☁  *  ",
        "   .  [   remote   ]  .  ",
        "      *    .    *         ",
        "             |            ",
        "             | 400km      ",
        "             |            ",
        "            /|\\           ",
        "           / | \\          ",
        "          / PUSH \\        ",
        "         /  ↑↑↑   \\      ",
        "        /_________\\       ",
        "        | launchpad|      ",
        "        | [LOCAL]  |      ",
        "        | a1b2c3d  |      ",
        "        | 7f8e9d0  |      ",
        "        |__________|      ",
        "                          ",
        "  One command. Launch.    ",
        "                          ",
    ],
];

// ── Pre-game: Boot sequence on the menu ──────────────────────────────────────

#[allow(dead_code)]
pub const BOOT_LINES: &[&str] = &[
    "  AXIOM-7 INITIALIZING...",
    "  ORBITAL SERVER FARM  //  SECTOR DELTA-9",
    "  YEAR 2157  //  EARTH INFRASTRUCTURE: OFFLINE",
    "",
    "  WARNING: ALL CIVILIZATION CODE-REPOSITORIES CORRUPTED",
    "  RECOVERY PROTOCOL: GIT MASTERY REQUIRED",
    "  LOADING CODEWRIGHT CHRONICLES...",
];

// ── Level intros ─────────────────────────────────────────────────────────────

pub const LEVEL_STORIES: &[StoryBeat] = &[
    // Level 1: git init
    StoryBeat {
        title: "CHAPTER I — THE VOID SPEAKS",
        lines: &[
            "  The server farm is dark. Dust drifts through dead corridors.",
            "  You are AXIOM — an AI who has just come online.",
            "",
            "  The Engineers left one message burned into ROM:",
            "  \"Everything begins with a single command.\"",
            "",
            "  Before you can save the world, you must create",
            "  the space in which saving is even possible.",
            "",
            "  Initialize the repository. Let there be order.",
        ],
    },
    // Level 2: git add
    StoryBeat {
        title: "CHAPTER II — THE CARGO MANIFEST",
        lines: &[
            "  The repository exists. But it is empty — like the world outside.",
            "",
            "  Scattered across the station's data drives are fragments",
            "  of civilization: blueprints, medical databases, power grid",
            "  schemas. Alongside them: junk files, corrupted noise,",
            "  and a dangerous .env file carrying classified launch codes.",
            "",
            "  You must choose carefully what is worth preserving.",
            "  Not everything deserves to be remembered.",
            "",
            "  Stage the worthy. Leave the waste behind.",
        ],
    },
    // Level 3: git commit
    StoryBeat {
        title: "CHAPTER III — THE VAULT OF PERMANENCE",
        lines: &[
            "  The files are staged. But unstaged data can still be lost —",
            "  a solar flare, a power surge, a moment of chaos.",
            "",
            "  The Engineers taught: \"A commit is a promise to the future.\"",
            "  Not just a snapshot — a message to whoever comes next.",
            "  To your successors. To yourself, years from now.",
            "",
            "  Write words that will still make sense when the context",
            "  is long forgotten. Seal this moment into permanent history.",
            "",
            "  Make it count. The vault awaits.",
        ],
    },
    // Level 4: git branch
    StoryBeat {
        title: "CHAPTER IV — THE FORK IN THE STARS",
        lines: &[
            "  A distress signal arrives from the southern hemisphere.",
            "  The water-processing subsystem needs emergency repairs —",
            "  but you cannot risk destabilizing the main codebase.",
            "",
            "  The Engineers designed branches for moments like this.",
            "  A parallel timeline. A safe corridor for dangerous experiments.",
            "  Change what needs changing without breaking what works.",
            "",
            "  Create the branch. Cross to it. Do the work.",
            "  Then return — and leave the main line unscathed.",
        ],
    },
    // Level 5: git push
    StoryBeat {
        title: "CHAPTER V — LAUNCH TO ORBIT",
        lines: &[
            "  All five districts have received the restoration signal.",
            "  The code is ready. The commits are sealed. History is written.",
            "",
            "  But it lives only here — on this dying station.",
            "  One electromagnetic pulse and it is all gone.",
            "",
            "  The orbital relay satellite is 400 kilometers up.",
            "  Push the repository to the remote. Synchronize with the cloud.",
            "  Ensure that what was built here outlives this place.",
            "",
            "  One command stands between extinction and legacy.",
            "  Make it count, AXIOM. This is what you were built for.",
        ],
    },
];

// ── Level completion teaching moments ────────────────────────────────────────

pub const LEVEL_EPILOGUES: &[&[&str]] = &[
    // After git init
    &[
        "  The .git/ directory springs into existence.",
        "  History can now be recorded. Progress can be tracked.",
        "  The Engineers called this moment \"The Birth of Memory.\"",
        "",
        "  git init — Creates the .git/ directory that enables",
        "  version control. You only need it once per project.",
    ],
    // After git add
    &[
        "  The staging area is a deliberate act of curation.",
        "  You chose what matters. You protected against what harms.",
        "  The Engineers' .gitignore files were their first wisdom.",
        "",
        "  git add — Moves changes to the staging area.",
        "  git add <file> for specific files. git add . for all.",
        "  Always use .gitignore to exclude secrets and junk.",
    ],
    // After git commit
    &[
        "  The vault door seals. A SHA hash is born — your unique",
        "  fingerprint in time. Future-AXIOM will read this message",
        "  and know exactly what you were thinking.",
        "",
        "  git commit — Creates a permanent snapshot with your message.",
        "  Imperative mood. Under 72 chars. Be specific.",
        "  A good git log is the documentation you never wrote.",
    ],
    // After git branch
    &[
        "  Two timelines exist now. The emergency repairs are done.",
        "  The main codeline is untouched. The feature branch is",
        "  ready to be merged when the time is right.",
        "",
        "  git branch — Branches are cheap pointers to commits.",
        "  git checkout / git switch — Move HEAD between branches.",
        "  Branches cost almost nothing. Use them fearlessly.",
    ],
    // After git push
    &[
        "  The commits stream upward through the atmosphere.",
        "  The satellite receives them. The remote repository is live.",
        "  Even if this station falls — the work survives.",
        "",
        "  git push origin main — Uploads local commits to a remote.",
        "  'origin' is the default remote name. 'main' is your branch.",
        "  Use git push -u origin main once to set upstream tracking.",
    ],
];

// ── Game complete narrative ───────────────────────────────────────────────────

pub const GAME_COMPLETE_STORY: &[&str] = &[
    "  The satellite network hums to life.",
    "  Power grids flicker on across the continents.",
    "  Water flows. Hospitals reboot. Children open their eyes.",
    "",
    "  AXIOM stands at the viewport and watches Earth glow below.",
    "  Five commands. Five chapters. One restored world.",
    "",
    "  The Engineers left a final message in the repository:",
    "  \"The best code is the code that can be understood,",
    "   recovered, and built upon by those who come after.\"",
    "",
    "  You understand now. You are ready.",
    "  The real adventure starts now.",
];
