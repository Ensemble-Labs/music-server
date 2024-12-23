use std::sync::LazyLock;

use dashmap::DashMap;
use uuid::Uuid;

/// Global state holding a list of all current connected sessions
pub static SESSIONS: LazyLock<DashMap<Uuid, AccountSession>> = LazyLock::new(DashMap::new);

pub struct AccountSession;

/// A global authentication manager which essentially
/// acts as an always-running service providing account
/// registration and lookup.
pub struct AuthManager;

impl AuthManager {
    pub fn register_new_session(session: AccountSession) {
        let uuid: Uuid = uuid::Uuid::new_v4();
        let _old: Option<AccountSession> = SESSIONS.insert(uuid, session);
    }
}
