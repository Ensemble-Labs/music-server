use std::{ffi::OsString, io, path::Path};
use id3::{Tag, TagLike, Timestamp};

/// Internal struct that holds the results of a parsed song name. This is
/// different from [Song], as that represents a song in the server's
/// database and has already been matched with a MusicBrainz result. In
/// contrast, this struct simply stores the result of trying to guess song
/// data from a string name. This may not always be 100% accurate.
#[derive(Default, Debug)]
pub struct ParsedSong {
    year: usize,
    name: String,
    artist: String,
    features: Vec<String>,
}

pub fn scan_dir(dir: &Path) -> io::Result<Vec<ParsedSong>> {
    let mut result: Vec<ParsedSong> = Vec::new();

    for entry in dir.read_dir()? {
        let entry = entry?;
        let file_name: OsString = entry.file_name();
        let path = entry.path();

        if let Some(parsed_song) = parse_local_id3(&path) {
            // result.push(parsed_song);
        }
    }

    Ok(result)
}

struct PartialID3Parse {
    title: Option<String>,
    artist: Option<String>,
    artists: Option<Vec<String>>,
    album: Option<String>,
    date: Option<Timestamp>,
    duration: Option<u32>,
    genre: Option<String>,
}

/// Parsing stage 1
/// Local tags are prioritized above all other parsed information.
fn parse_local_id3(path: &Path) -> Option<PartialID3Parse> {
    let tag: Tag = Tag::read_from_path(path).ok()?;
    Some(
        PartialID3Parse {
            title: tag.title().map(|s| s.to_string()),
            artist: tag.artist().map(|s| s.to_string()),
            artists: tag.artists().map(|s| s.iter().map(|s| s.to_string()).collect()),
            album: tag.album().map(|s| s.to_string()),
            date: tag.date_released(),
            duration: tag.duration(),
            genre: tag.genre_parsed().map(|s| s.to_string()),
        }
    )
}
