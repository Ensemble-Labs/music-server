//! The lib.rs file is mostly used to organize the crate's namespace and
//! provide utility functions/macros.

// Declare rust module tree
pub mod auth;
pub mod db;
pub mod responders;

// Re-export commonly used types closer to crate root
pub mod types {
    pub use crate::db::accounts::AccountRecord;
}
