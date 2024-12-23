//! Abstraction to handle filesystem details of
//! the in-memory DB store

use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use serde::Serialize;

// struct DbFileManager {
//     file: File,
// }

static DB_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path: PathBuf = dirs::data_dir().expect("Unsupported operating system!");
    path.push(".orpheus/accounts-db");
    if !path.exists() {
        std::fs::write(&path, const { [0_u8] }).expect("Failed to write to account db file!");
    }
    path
});

pub fn get_accdb_file() -> &'static Path {
    DB_FILE_PATH.as_ref()
}

pub fn write_accdb(buf: &impl Serialize) -> anyhow::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(buf)?;
    let path: &'static Path = DB_FILE_PATH.as_ref();
    std::fs::write(path, &encoded)?;
    Ok(())
}
