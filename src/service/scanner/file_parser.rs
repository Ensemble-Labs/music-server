use std::{ffi::OsString, path::Path};

use tracing::warn;

/// Internal struct that holds the results of a parsed song name. This is
/// different from [Song], as that represents a song in the server's
/// database and has already been matched with a MusicBrainz result. In
/// contrast, this struct simply stores the result of trying to guess song
/// data from a string name. This may not always be 100% accurate.
#[derive(Default, Debug)]
struct ParseSong {
    year: usize,
    name: String,
    artist: String,
    features: Vec<String>,
}

/// Parses a single file name and tries to extract basic song data from it
/// that can be used for MusicBrainz matching.
#[cfg(target_os = "none")] // musicbrainz matching comes later
fn try_parse_song(raw: &str) -> Option<ParseSong> {
    let mut found_year = false;
    let mut year = 0;
    let mut data: ParseSong = ParseSong::default();
    todo!()
}

/// Small private function to obtain the file extension of a string.
fn file_extension(path: &str) -> Option<&str> {
    let max: usize = path.len() - 1; // avoid repeated calls to len
    for (i, c) in path.chars().enumerate() {
        if c == '.' && i != max {
            return Some(&path[(i + 1)..]);
        }
    }
    None
}

pub fn scan_dir(dir: &Path) -> std::io::Result<()> {
    for entry in dir.read_dir()? {
        let file_name: OsString = entry?.file_name();
        // overshadow the name `file_name`
        let Some(file_name) = file_name.to_str() else {
            warn!(?file_name, "Failed to decode file name into UTF-8!");
            continue;
        };

        if let Some(ext) = file_extension(file_name) {
            match ext {
                "mp3" => todo!("mp3 tag reader"),
                "mp4" | "m4a" => todo!("mp4 tag reader"),
                "flac" => todo!("flac tag reader"),
                _ => tracing::warn!(?ext, "Unrecognized file extension."),
            }
        } else {
            warn!(?file_name, "File is missing an extension!");
        }
    }
    todo!()
}
