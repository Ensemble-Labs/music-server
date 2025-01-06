use std::sync::{Arc, LazyLock};

use crate::types::AccountRecord;
use chrono::prelude::*;
use papaya::HashMap;
use uuid::Uuid;

// Simple strong type around Uuid for clarity
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Token(pub Uuid);

impl Token {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TryFrom<&str> for Token {
    type Error = uuid::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(Uuid::parse_str(value)?))
    }
}

/// global variable holding the singleton instance of [AuthManager].
pub static SESSIONS: LazyLock<AuthManager> = LazyLock::new(AuthManager::start);

/// Holds information about a logged in account during its
/// current session.
pub struct AccountSession {
    record: AccountRecord,
    token: Token,
    started: DateTime<Utc>,
    expires: DateTime<Utc>,
}

impl AccountSession {
    pub fn record(&self) -> &AccountRecord {
        &self.record
    }

    pub fn token(&self) -> Token {
        self.token
    }

    pub fn started(&self) -> DateTime<Utc> {
        self.started
    }

    pub fn expires(&self) -> DateTime<Utc> {
        self.expires
    }

    pub fn is_expired(&self) -> bool {
        self.started() >= self.expires()
    }
}

/// A global authentication manager which handles logging in users and
/// authenticating their requests via session tokens. This struct uses
/// UUID v4s as session tokens, which are issued upon a successful login and
/// stored in the account record. A user authenticates themselves per-action
/// by providing their session token along with their username.
pub struct AuthManager {
    /// A hash table mapping usernames to their respective session instances.
    sessions: Arc<HashMap<String, AccountSession>>,
}

// Mark types as safe to send since all methods use thread-safe
// operations (operating on [Arc]s)
unsafe impl Send for AuthManager {}
unsafe impl Sync for AuthManager {}

impl AuthManager {
    // Constructor //
    pub fn start() -> Self {
        Self {
            sessions: Arc::new(HashMap::new()),
        }
    }

    // Methods //
    /// Registers a given [AccountSession] into the global session table. This
    /// method will return [true] if the account was successfully logged in, and [false]
    /// if the account was already logged in and it's session has not yet expired.
    fn register_new_session(&self, session: AccountSession) -> bool {
        let sessions = self.sessions.clone();
        let map = sessions.pin();
        let name: &str = session.record().username();

        if !map.contains_key(name) || map.get(name).unwrap().is_expired() {
            map.insert(session.record().username().to_owned(), session);
            true
        } else {
            false
        }
    }

    /// Attempts to authenticate a user's credentials by ensuring they have the
    /// correct session token for their username.
    pub fn authenticate(&self, username: &str, token: Token) -> bool {
        self.sessions
            .clone()
            .pin()
            .get(username)
            .is_some_and(|u| u.token() == token)
    }
}
