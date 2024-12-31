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
    alloc::{alloc, Layout},
    path::{Path, PathBuf},
    sync::{Arc, LazyLock, Mutex},
};
use tracing::{debug, trace};

use crate::services;

/// A global concurrent hashmap storing a list of all accounts read in from disk.
///
/// TODO:
/// - Consider changing to a frozen map
#[allow(non_upper_case_globals)] // i like the "*Service" naming scheme, sue me
pub static AccountService: LazyLock<AccountsManager> = LazyLock::new(|| {
    let data_path = PathBuf::from(
        services::Config
            .try_read()
            .unwrap()
            .server()
            .account_data_path(),
    );
    AccountsManager::from_path(data_path)
});

/// A small data struct to hold information about an account. Username is a duplicate
/// field here despite also being used as the key to the HashMap.
#[derive(Serialize, Deserialize, Debug)]
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
    dirty: Mutex<bool>,
    accounts: Arc<DashMap<String, AccountRecord>>,
}

unsafe impl Send for AccountsManager {}
unsafe impl Sync for AccountsManager {}

impl AsRef<AccountsManager> for AccountsManager {
    fn as_ref(&self) -> &AccountsManager {
        self
    }
}

impl AccountsManager {
    // Constructors //
    pub fn create(to_path: impl std::fmt::Display, map: DashMap<String, AccountRecord>) -> Self {
        let path: PathBuf = PathBuf::from(to_path.to_string());
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p)
                .expect("Failed to create data file path! Double check write permissions.");
        }

        let s = Self {
            path,
            dirty: Mutex::new(false),
            accounts: Arc::new(map),
        };
        s.save(); // test if writing crashes so the user doesn't find out when it's too late
        s
    }

    pub fn from_path(path: PathBuf) -> Self {
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p)
                .expect("Failed to create data file path! Double check write permissions.");
        }

        let accounts: DashMap<String, AccountRecord> = if !path.exists() {
            DashMap::new()
        } else {
            let contents: String =
                std::fs::read_to_string(&path).expect("Failed to read data file!");
            bincode::deserialize(contents.as_bytes())
                .expect("Failed to deserialized account data file!")
        };

        let new: Self = Self {
            path,
            dirty: Mutex::new(false),
            accounts: Arc::new(accounts),
        };
        trace!(
            "Creating account manager with table: {:?}",
            new.accounts.clone()
        );
        new.save();
        new
    }

    // Methods //
    pub fn save(&self) {
        *self.dirty.lock().unwrap() = false;
        trace!(
            "Saving accounts database with table: {:?}",
            &self.accounts.clone()
        );
        let encoded: Vec<u8> = bincode::serialize(self.accounts.as_ref())
            .expect("Failed to serialize accounts storage!");
        let path: &Path = self.path.as_ref();
        std::fs::write(path, &encoded).expect("Failed to save to accounts DB path!");
    }

    pub fn register_from_record(&self, record: AccountRecord) -> Result<()> {
        *self.dirty.lock().unwrap() = true;
        let map = self.accounts.clone();
        if !map.contains_key(&record.username) {
            debug!(
                "Registering account {{ username: {}, password: {} }}",
                &record.username, &record.password_hash
            );
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

    pub fn is_dirty(&self) -> bool {
        *self.dirty.lock().unwrap()
    }

    /// This function does nothing. It exists only to force the lazy initialization
    /// of the [LazyLock] holding the global [AccountsManager].
    pub fn verify(&self) {}
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
