//! # Server Authentication
//! This module provides functionality to authenticate user sessions. Note that
//! actual user accounts (registration and management) is handled by the `accounts`
//! module, not auth. The scope of this module is logging into a server, managing
//! currently running sessions, and to verify details like user's permissions.

use std::sync::{Arc, LazyLock};

use crate::types::LoginCode;
use crate::{services::AccountService, types::AccountRecord};
use axum::response::IntoResponse;
use chrono::{prelude::*, TimeDelta};
use papaya::HashMap;
use uuid::Uuid;

const SESSION_EXPIRY: TimeDelta = TimeDelta::hours(6);

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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoResponse for Token {
    fn into_response(self) -> axum::response::Response {
        self.to_string().into_response()
    }
}

/// global variable holding the singleton instance of [AuthManager].
pub static SESSIONS: LazyLock<AuthManager> = LazyLock::new(AuthManager::start);

/// Holds information about a logged in account during its
/// current session.
pub struct AccountSession {
    record: Arc<AccountRecord>,
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
    sessions: Arc<HashMap<String, Arc<AccountSession>>>,
}

// Mark types as safe to send since all methods use thread-safe
// operations (operating on [Arc]s)
unsafe impl Send for AuthManager {}
unsafe impl Sync for AuthManager {}

/// Basic wrapper strong-type around the 3 possible login results.
/// This is used in `AuthManager` to return an account session, and is not
/// necessary in the login function for `AccountsManager` as that isn't
/// API-facing.
pub enum AuthCode {
    Success(Arc<AccountSession>),
    InvalidPassword,
    AccountNotFound,
}

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
    fn register_new_session(&self, session: Arc<AccountSession>) -> bool {
        let sessions = self.sessions.clone();
        let map = sessions.pin();
        let name: &str = session.record().username();

        tracing::debug!(?name, "attempting to register session");
        if !map.contains_key(name) || map.get(name).unwrap().is_expired() {
            tracing::debug!(
                "registered session for {name} with {}",
                session.token().0.to_string()
            );
            map.insert(name.to_owned(), session);
            true
        } else {
            false
        }
    }

    /// TODO:
    /// - Add customizable session expiry (current +6 hours)
    pub fn login(&self, username: &str, password: &str) -> AuthCode {
        match AccountService.login(username, password) {
            LoginCode::Success(record) => {
                let now = Utc::now();
                let session = AccountSession {
                    record,
                    token: Token::generate(),
                    started: now,
                    expires: now + SESSION_EXPIRY,
                };
                let sr: Arc<AccountSession> = Arc::new(session);
                self.register_new_session(sr.clone());
                AuthCode::Success(sr.clone())
            }
            LoginCode::InvalidPassword => AuthCode::InvalidPassword,
            LoginCode::AccountNotFound => AuthCode::AccountNotFound,
        }
    }

    /// Attempts to authenticate a user's credentials by ensuring they have the
    /// correct session token for their username.
    pub fn auth_get_session(&self, username: &str, token: Token) -> Option<Arc<AccountSession>> {
        let map = self.sessions.clone();
        let guard = map.pin();
        let record = guard.get(username);
        if record.is_some_and(|u| u.token() == token) {
            Some(record.unwrap().clone())
        } else {
            None
        }
    }
}
