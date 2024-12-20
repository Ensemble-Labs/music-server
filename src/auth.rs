use std::sync::LazyLock;

use dashmap::DashMap;
use uuid::Uuid;

/// Global state holding a list of all current connected sessions
pub static SESSIONS: LazyLock<DashMap<Uuid, AccountSession>> = LazyLock::new(DashMap::new);

pub struct AccountSession;

/// A global authentication manager which essentially
/// acts as an always-running daemon providing account
/// registration and lookup.
pub struct AuthManager;

impl AuthManager {}
