//! Module to manage the storage and retrieval of
//! accounts to/from disk.

use anyhow::{bail, Result};
use dashmap::DashMap;
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Scrypt,
};
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

/// A thread-safe in-memory account database. It is initialized by providing a path to
/// a database file, one it will either create or read depending on the constructor used.
pub struct AccountsManager {
    path: PathBuf,
    accounts: Arc<DashMap<String, AccountRecord>>,
}

unsafe impl Send for AccountsManager {}
unsafe impl Sync for AccountsManager {}

impl AccountsManager {
    // Constructors //
    pub fn create(to_path: impl std::fmt::Display, map: DashMap<String, AccountRecord>) -> Self {
        let path: PathBuf = PathBuf::from(to_path.to_string());
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p)
                .expect("Failed to create data file path! Double check write permissions.");
        }
        std::fs::write(&path, [0_u8; 0]).expect("Error writing to db file!");
        Self {
            path,
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

    pub fn register_from_record(&self, record: AccountRecord) -> Result<()> {
        let map = self.accounts.clone();
        if !map.contains_key(&record.username) {
            map.insert(record.username.clone(), record);
            Ok(())
        } else {
            bail!("Account already exists!")
        }
    }

    pub fn register(&self, username: String, password: String, is_admin: bool) -> Result<()> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        let record: AccountRecord = AccountRecord {
            username,
            password_hash,
            is_admin,
        };
        self.register_from_record(record)
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
