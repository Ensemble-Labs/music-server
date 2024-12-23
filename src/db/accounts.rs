//! Module to manage the storage and retrieval of
//! accounts to/from disk.

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    sync::LazyLock,
};

pub static ACCOUNTS: LazyLock<DashMap<String, String>> = LazyLock::new(DashMap::new);

#[derive(Serialize, Deserialize)]
pub struct AccountsManager {
    #[serde(skip)]
    path: PathBuf,
    accounts: DashMap<String, String>,
}

unsafe impl Send for AccountsManager {}
unsafe impl Sync for AccountsManager {}

// Constructors
impl AccountsManager {
    // Constructors
    pub fn create(to_path: impl Display) -> Self {
        Self {
            path: PathBuf::from(to_path.to_string()),
            accounts: DashMap::new(),
        }
    }

    pub fn at(path: PathBuf) -> Self {
        let bytes: Vec<u8> = std::fs::read(&path).expect("Failed to read accounts DB path!");
        let mut new_self: Self =
            bincode::deserialize(&bytes).expect("Accounts database file corrupted!");
        new_self.path = path;
        new_self
    }

    // Methods
    pub fn save(&self) {
        let encoded: Vec<u8> =
            bincode::serialize(&self).expect("Failed to serialize accounts storage!");
        let path: &Path = self.path.as_ref();
        std::fs::write(path, &encoded).expect("Failed to save to accounts DB path!");
    }
}

impl Drop for AccountsManager {
    fn drop(&mut self) {
        self.save();
    }
}
