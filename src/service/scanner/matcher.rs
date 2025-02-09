use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    name: String,
    icon_url: String,
    description: String,
    genres: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    name: String,
    artist: Arc<Artist>,
    features: Option<Vec<Arc<Artist>>>,
    songs: Vec<Arc<Song>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
    artist: Arc<Artist>,
    album: Arc<Album>,
    features: Option<Vec<Arc<Artist>>>,
    name: String,
    genres: Vec<String>,
    duration: u32,
}
