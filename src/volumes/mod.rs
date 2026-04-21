// volumes/mod.rs — GitQuest story content, chapter data, and public API.

pub mod story;
pub use story::{all_volumes, rank_title, Chapter, Volume};
