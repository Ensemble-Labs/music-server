//! Module to manage the storage and retrieval of
//! accounts to/from disk.

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
};

/// A global concurrent hashmap storing a list of all accounts read in from disk.
///
/// TODO:
/// - Consider changing to a frozen map
#[allow(non_upper_case_globals)] // i like the "*Service" naming scheme, sue me
pub static AccountService: LazyLock<AccountsManager> =
    LazyLock::new(|| todo!("a config module to store filedb path"));

/// A small data struct to hold information about an account. Username is a duplicate
/// field here despite also being used as the key to the HashMap.
#[derive(Serialize, Deserialize)]
pub struct AccountRecord {
    username: String,
    password_hash: String,
    /// can the user manage the server (i.e. create new accounts?)
    is_admin: bool,
}

pub struct AccountsManager {
    path: PathBuf,
    accounts: Arc<DashMap<String, AccountRecord>>,
}

unsafe impl Send for AccountsManager {}
unsafe impl Sync for AccountsManager {}

impl AccountsManager {
    // Constructors //
    pub fn create(to_path: impl std::fmt::Display, map: DashMap<String, AccountRecord>) -> Self {
        Self {
            path: PathBuf::from(to_path.to_string()),
            accounts: Arc::new(map),
        }
    }

    pub fn at(path: PathBuf) -> Self {
        let bytes: Vec<u8> = std::fs::read(&path).expect("Failed to read accounts DB path!");
        let accounts: DashMap<String, AccountRecord> =
            bincode::deserialize(&bytes).expect("Accounts database file corrupted!");
        Self {
            path,
            accounts: Arc::new(accounts),
        }
    }

    // Methods //
    pub fn save(&self) {
        let encoded: Vec<u8> = bincode::serialize(self.accounts.as_ref())
            .expect("Failed to serialize accounts storage!");
        let path: &Path = self.path.as_ref();
        std::fs::write(path, &encoded).expect("Failed to save to accounts DB path!");
    }

    pub fn register_hashed(record: AccountRecord) {
        todo!()
    }
}

/// # Why manually implement drop for this type?
/// There's a lot of solutions to solve the problem of "when exactly do we save?"
/// Typically the solution reached is allowing manual saving + auto-saving at
/// every set interval or action. However, since Rust will call
/// every struct's drop implementation, on graceful exit or on panic, we can
/// simply just delegate the saving to disk then. Ideally we'll implement more frequent
/// saves than that though, to account for power outages and [std::process::exit] calls.
impl Drop for AccountsManager {
    fn drop(&mut self) {
        self.save();
    }
}
