//! The lib.rs file is the crate's library root and is used to organize
//! the crate's namespace, hold unit tests, andprovide utility functions/macros.

use std::sync::{Arc, LazyLock};

// declare rust module tree
pub mod auth;
pub mod config;
pub mod db;
pub mod responders;

pub static CONFIG: LazyLock<Arc<config::Config>> = LazyLock::new(|| {
    Arc::new(
        toml::from_str(include_str!("../orpheus-EXAMPLE.toml")).expect("Malformed `orpheus.toml`!"),
    )
});

// re-export commonly used types closer to crate root
pub mod types {
    pub use crate::db::accounts::AccountRecord;
}

// re-export all services for ease of use
pub mod services {
    pub use crate::db::accounts::AccountService;
}

// unit testing
#[cfg(test)]
mod tests {
    use toml::Table;

    #[test]
    pub fn dbg_example_toml() {
        let cfg_toml: Table = include_str!("../orpheus-EXAMPLE.toml")
            .parse::<Table>()
            .unwrap();
        dbg!(cfg_toml);
    }
}
