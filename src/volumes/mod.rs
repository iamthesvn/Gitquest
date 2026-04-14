// volumes/mod.rs — GitQuest story content, chapter data, and public API.
//
// This IS the `volumes` module (Rust uses volumes/mod.rs over volumes.rs
// when both exist — but since the project has volumes.rs as well, Rust 2021
// requires only one to exist.  All story content lives in story.rs which is
// declared below so `crate::volumes::Chapter` etc. resolve correctly.

pub mod story;
pub use story::{all_volumes, rank_title, Chapter, Volume};
