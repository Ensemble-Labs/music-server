//! Module to manage the storage and retrieval of
//! accounts to/from disk.

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    sync::LazyLock,
};

/// A global concurrent hashmap storing a list of all accounts read in from disk.
///
/// TODO:
/// - Consider changing to a frozen map
pub static ACCOUNTS: LazyLock<DashMap<String, String>> = LazyLock::new(DashMap::new);

/// A small data struct to hold information about an account. The reason usernames are not
/// included in this record is we currently use usernames as the key to the account_db, rather than the
/// more typical implementation which would be user id. The rationale behind this is that we simply
/// do not need elegant account tracking as each server is most likely not going to be used by a
/// large amount of people, given the decentralized nature of Orpheus.
#[derive(Serialize, Deserialize)]
pub struct AccountRecord {
    password_hash: String,
    /// can the user manage the server (i.e. create new accounts?)
    is_admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AccountsManager {
    #[serde(skip)]
    path: PathBuf,
    accounts: DashMap<String, AccountRecord>,
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

/// # Why manually implement drop for this type?
/// There's a lot of solutions to solve the problem of "when exactly do we save?"
/// Typically the solution reached is allowing manual saving + auto-saving at
/// every set interval or action. However, since Rust guarantees it will call
/// every struct's drop implementation, on graceful exit or on panic, we can
/// simply just delegate the saving to disk then. Ideally, we'll implement more frequent
/// saves than that to account for power outages though.
impl Drop for AccountsManager {
    fn drop(&mut self) {
        self.save();
    }
}
