//! # Minigrep Crate
//!
//! `minigrep` is based on the Rust book's CLI miniproject. I've done trivial
//! things in my own way rather than follow it strictly just to demonstrate
//! it's not a pure copy-pasta job.

pub mod search;
pub mod config;


// re-exports directly to here: the crate's base module
pub use self::search::{search, search_insensitive};
pub use self::config::SearchConfig;
