use std::sync::{Arc, LazyLock};

use crate::types::AccountRecord;
use chrono::prelude::*;
use papaya::HashMap;
use uuid::Uuid;

// Simple strong type around Uuid for clarity
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Token(Uuid);

impl Token {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

/// global variable holding the singleton instance of [AuthManager].
pub static SESSIONS: LazyLock<AuthManager> = LazyLock::new(AuthManager::start);

/// Holds information about a logged in account during its
/// current session.
pub struct AccountSession {
    record: AccountRecord,
    started: DateTime<Utc>,
    expires: DateTime<Utc>,
}

impl AccountSession {
    pub fn record(&self) -> &AccountRecord {
        &self.record
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
/// stored in the session map. A user authenticates themselves per-action
/// by providing their session token along with their username.
pub struct AuthManager {
    /// A hash table mapping session IDs to their respective account records.
    sessions: Arc<HashMap<Token, AccountSession>>,
    /// A hash table mapping logged-in users' names to their session IDs.
    session_ids: Arc<HashMap<String, Token>>,
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
            session_ids: Arc::new(HashMap::new()),
        }
    }

    // Methods //
    /// Registers a given [AccountSession] into the global session table
    /// by generating a new [Token] and returning it.
    ///
    /// TODO:
    /// Check if account is already logged in, and if it is (and its session has
    /// not expired), return None.
    pub fn register_new_session(&self, session: AccountSession) -> Option<Token> {
        let token: Token = Token::generate();
        self.sessions.clone().pin().insert(token, session);
        Some(token)
    }

    /// Attempts to authenticate a user's credentials by ensuring they have the
    /// correct session token for their username.
    pub fn authenticate(&self, username: &str, token: &Token) -> bool {
        self.session_ids
            .clone()
            .pin()
            .get(username)
            .is_some_and(|u| u == token)
    }
}
