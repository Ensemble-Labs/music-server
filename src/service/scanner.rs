//! # Song Scanner
//! Reads songs put in the server's song directory and attempts to
//! obtain metadata about them to better display song information
//! in Orpheus clients. This metadata is available upon request for any
//! song ID.
//!
//! Note that in this module, song metadata priority is sorted by:
//! 1. First analyzing the tags on the audio file itself,
//! 2. Then attempting to match the file name with a MusicBrainz result
//! This means local metadata will always take priority, as it is assumed to
//! be verified by a human.

mod file_parser;
mod matcher;
pub use matcher::{Album, Artist, Song};
