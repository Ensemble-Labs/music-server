use std::sync::{Arc, LazyLock};

use crate::types::AccountRecord;
use chrono::prelude::*;
use dashmap::DashMap;
use uuid::Uuid;

/// global state holding a list of all current connected sessions
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
}

/// a global authentication manager which essentially
/// acts as an always-running service providing account
/// registration and lookup.
pub struct AuthManager {
    sessions: Arc<DashMap<Uuid, AccountSession>>,
    session_ids: Arc<DashMap<String, Uuid>>,
}

// Mark types as safe to send since all methods use thread-safe
// operations (operating on [Arc]s)
unsafe impl Send for AuthManager {}
unsafe impl Sync for AuthManager {}

impl AuthManager {
    // Constructor //
    pub fn start() -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
            session_ids: Arc::new(DashMap::new()),
        }
    }

    // Methods //
    pub fn register_new_session(&self, session: AccountSession) -> Option<AccountSession> {
        let uuid: Uuid = uuid::Uuid::new_v4();
        self.sessions.clone().insert(uuid, session)
    }

    pub fn authenticate(&self, username: &str, uuid: &Uuid) -> bool {
        self.session_ids
            .clone()
            .get(username)
            .is_some_and(|u| u.value() == uuid)
    }
}
