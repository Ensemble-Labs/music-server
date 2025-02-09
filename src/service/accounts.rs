//! # Server Accounts
//! Manages the storage and retrieval of accounts to/from disk. This module
//! focuses on loading the account database, registering/deleting accounts,
//! and saving the database back to file.

use anyhow::{bail, Result};
use papaya::HashMap;
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock, Mutex},
};
use tracing::{debug, trace};

use crate::services;

/// Global variable holding the singleton instance of [AccountsManager].
///
/// TODO:
/// - Consider changing to a frozen map
#[allow(non_upper_case_globals)] // i like the "*Service" naming scheme, sue me
pub static AccountService: LazyLock<AccountsManager> = LazyLock::new(|| {
    let data_path = PathBuf::from(
        services::Config
            .try_read() // we immediately try to acquire the lock as this is startup
            .unwrap() // immediately unwrap said lock since nothing else is locking yet
            .server()
            .account_data_path(), // see key [server.account_data_path] in `orpheus.toml`
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

#[derive(Serialize, Deserialize)]
struct AccountData {
    // all fields commented out need their respective types to be
    // implemented before they can be uncommented.
    // playlists: UserPlaylists,
    // stats: StatRecorder,
}

crate::make_getters!(AccountRecord, username: String, password_hash: String, is_admin: bool);

/// A thread-safe in-memory account database. It is initialized by providing a path to
/// a database file, one it will either create or read depending on the constructor used.
pub struct AccountsManager {
    path: PathBuf,
    dirty: Mutex<bool>,
    accounts: Arc<HashMap<String, Arc<AccountRecord>>>,
}

// Explicitly mark [AccountsManager] as thread-safe since all operations
// are behind [Arc]s and thread-safe structs.
unsafe impl Send for AccountsManager {}
unsafe impl Sync for AccountsManager {}

impl AsRef<AccountsManager> for AccountsManager {
    fn as_ref(&self) -> &AccountsManager {
        self
    }
}

pub enum LoginCode {
    Success(Arc<AccountRecord>),
    InvalidPassword,
    AccountNotFound,
}

impl AccountsManager {
    // Constructors //
    pub fn create(to_path: impl Display, map: HashMap<String, Arc<AccountRecord>>) -> Self {
        let path: PathBuf = PathBuf::from(to_path.to_string()); // convert generic parameter to [String]
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p) // make all necessary directories to create data file
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
            std::fs::create_dir_all(p) // make all necessary directories to create data file
                .expect("Failed to create data file path! Double check write permissions.");
        }

        let accounts: HashMap<String, Arc<AccountRecord>> = if !path.exists() {
            HashMap::new() // if there's no file at path, make a new map
        } else {
            let contents: Vec<u8> = std::fs::read(&path).expect("Failed to read data file!");
            pot::from_slice(contents.as_slice()).expect("Failed to deserialize account data file!")
        };

        let new: Self = Self {
            path,
            dirty: Mutex::new(false),
            accounts: Arc::new(accounts),
        };
        trace!("Creating account manager with table: {:?}", new.accounts);
        new.save();
        new
    }

    // Methods //
    /// Unmarks the struct as dirty and saves the entire contents
    /// to the file path provided on creation of the struct.
    pub fn save(&self) {
        *self.dirty.lock().unwrap() = false; // set self.dirty to false
        trace!("Saving accounts database with table: {:?}", self.accounts);
        let encoded: Vec<u8> =
            pot::to_vec(self.accounts.as_ref()).expect("Failed to serialize accounts storage!");
        let path: &Path = self.path.as_ref();
        std::fs::write(path, &encoded).expect("Failed to save to accounts DB path!");
    }

    /// Uploads an account record directly to the map, using a clone of
    /// its `username` field as the key.
    pub fn register_from_record(&self, record: AccountRecord) -> Result<()> {
        *self.dirty.lock().unwrap() = true;
        let amap = self.accounts.clone(); // obtain atomic reference to map
        let map = amap.pin(); // lock map's memory from being freed
        if !map.contains_key(&record.username) {
            debug!(
                "Registering account {{ username: {}, password: {} }}",
                &record.username, &record.password_hash
            );
            map.insert(record.username.clone(), Arc::new(record));
            Ok(())
        } else {
            bail!("Account already exists!")
        }
    }

    /// Registers from record without checking if it already exists. Used only
    /// in the [AccountsManager::register] function to skip redundant checking.
    fn register_from_record_unchecked(&self, record: AccountRecord) {
        *self.dirty.lock().unwrap() = true;
        let map = self.accounts.clone();
        debug!(
            "Registering account {{ username: {}, password: {} }}",
            &record.username, &record.password_hash
        );
        map.pin().insert(record.username.clone(), Arc::new(record));
    }

    /// Creates a new entry in the account registry with:
    /// 1. the username as the key,
    /// 2. and an [AccountRecord] containing a clone of the username,
    ///    the password, and whether or not the account is an admin.
    pub fn register(&self, username: String, password: String, is_admin: bool) -> Result<()> {
        let salt = SaltString::generate(&mut OsRng); // generate salt for the password hash
        let map = self.accounts.clone(); // obtain reference to map
        let password_hash = if !map.pin().contains_key(&username) {
            Scrypt // return newly hashed password if not already registered
                .hash_password(password.as_bytes(), &salt)?
                .to_string()
        } else {
            tracing::error!("Failed to register already-registered account \"{username}\"!");
            bail!("Account already exists!") // error on existing account
        };
        drop(map); // drop our reference to map as next function will reference it
        let record: AccountRecord = AccountRecord {
            username,
            password_hash,
            is_admin,
        };
        self.register_from_record_unchecked(record);
        Ok(())
    }

    /// Attempts to verify the provided password against the entry for the
    /// username provided. This function will either return:
    /// - `Some(true)` if the username and password are both valid and correct
    /// - `Some(false)` if the username is registered, but the password is incorrect
    /// - `None` if the username is not registered.
    ///
    /// Note that this function can also return `None` if the password hash fails to
    /// be read into the password hash format, however this shouldn't happen in
    /// practice and most likely doesn't need to be accounted for.
    pub fn login(&self, username: &str, password: &str) -> LoginCode {
        let amap = self.accounts.clone(); // obtain atomic reference to map
        let map = amap.pin(); // lock map's memory from being freed
        if let Some(record) = map.get(username).cloned() {
            let hash = PasswordHash::new(record.password_hash()).unwrap(); // parse hash (should never fail)
            if Scrypt.verify_password(password.as_bytes(), &hash).is_ok() {
                LoginCode::Success(record)
            } else {
                LoginCode::InvalidPassword
            }
        } else {
            LoginCode::AccountNotFound
        }
    }

    pub fn is_dirty(&self) -> bool {
        *self.dirty.lock().unwrap()
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
