//! The lib.rs file is the crate's library root and is used to organize
//! the crate's namespace, hold unit tests, andprovide utility functions/macros.

// declare rust module tree
pub mod auth;
pub mod config;
pub mod db;
pub mod responders;

// re-export commonly used types closer to crate root
pub mod types {
    pub use crate::db::accounts::AccountRecord;
}

// re-export all services for ease of use
pub mod services {
    pub use crate::config::CONFIG as Config;
    pub use crate::db::accounts::AccountService;
}

/// A macro to generate getters for every field of a struct.
///
/// Example:
/// ```rs
/// pub struct MyStruct {
///     field1: String,
///     field2: i32,
/// }
/// make_getters!(MyStruct, field1: String, field2: i32);
/// ```
macro_rules! make_getters {
    ($s:ident,$($x:ident:$t:ty),+) => { // dispatcher using recursion to emulate switch
        impl $s {
            $(make_getters!{ $x:$t })*
        }
    };
    ($x:ident:String) => { // special case 1: if referencing a `String`, return `&str` not `&String`
        pub fn $x(&self) -> &str {
            &self.$x
        }
    };
    ($x:ident:Vec<$t:ty>) => { // special case 2: instead of `&Vec<T>`, return `&[T]`
        pub fn $x(&self) -> &[$t] {
            &self.$x
        }
    };
    ($x:ident:$t:ty) => { // default case: just return `&T`
        pub fn $x(&self) -> &$t {
            &self.$x
        }
    };
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
