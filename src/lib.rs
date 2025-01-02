//! The lib.rs file is the crate's library root and is used to organize
//! the crate's namespace, hold unit tests, andprovide utility functions/macros.

// declare rust module tree
pub mod auth;
pub mod config;
pub mod db;
pub mod responders;

// `lib` folder module tree
// here we use custom paths because for some reason rust believes the
// `lib.rs` file and the `src/lib/` directory are completely unrelated concepts.
#[path = "lib/struct_utils.rs"]
mod struct_utils;

// re-export commonly used types closer to crate root
pub mod types {
    pub use crate::db::accounts::AccountRecord;
}

// re-export all services for ease of use
pub mod services {
    pub use crate::config::CONFIG as Config;
    pub use crate::db::accounts::AccountService;
}

// unit testing
#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::services::AccountService;
    use toml::Table;

    #[test]
    pub fn dbg_example_toml() {
        let cfg_toml: Table = include_str!("../orpheus-EXAMPLE.toml")
            .parse::<Table>()
            .unwrap();
        dbg!(cfg_toml);
    }

    #[test]
    pub fn test_read_config() {
        let cfg = crate::config::CONFIG.blocking_read();
        dbg!(cfg.server().account_data_path());
        let _ = dbg!(cfg);
    }

    #[test]
    pub fn test_change_config() {
        let mut cfg = crate::config::CONFIG.blocking_write();
        cfg.server_mut().set_account_data_path("data_path".into());
        std::fs::write("./orpheus-out.toml", cfg.output()).unwrap();
    }

    #[test]
    pub fn bench_saving_accounts() {
        AccountService.verify();
        let t: Instant = Instant::now();
        AccountService.save();
        println!("{:?}", t.elapsed());
    }
}
