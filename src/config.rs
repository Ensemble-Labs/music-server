use std::{path::PathBuf, sync::LazyLock};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Rust representation of the TOML config for Orpheus.
/// Example can be found in `orpheus-EXAMPLE.toml`.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    server: ServerConfig,
}

impl Config {
    // Getters & Setters
    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn server_mut(&mut self) -> &mut ServerConfig {
        &mut self.server
    }
}

impl Config {
    // Methods
    pub fn save(&self) {
        let path: PathBuf = std::env::current_dir()
            .expect("Failed to access current directory!")
            .join("orpheus.toml");
        std::fs::write(
            &path,
            toml::to_string_pretty(&self).expect("Failed to serialize Config!"),
        )
        .expect("Failed to write to `orpheus.toml`!");
    }

    pub fn output(&self) -> String {
        toml::to_string_pretty(&self).expect("Failed to serialize Config!")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    account_data_path: String,
    bind_address: String,
}

impl ServerConfig {
    pub fn account_data_path(&self) -> &str {
        &self.account_data_path
    }

    pub fn set_account_data_path(&mut self, data_path: String) {
        self.account_data_path = data_path;
    }

    pub fn bind_address(&self) -> &str {
        &self.bind_address
    }
}

// Global config store from file
pub static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| {
    let path: PathBuf = std::env::current_dir()
        .expect("Failed to access current directory!")
        .join("orpheus.toml");
    let toml_file: String = std::fs::read_to_string(&path).expect("Failed to read `orpheus.toml`!");
    RwLock::new(toml::from_str(&toml_file).expect("`orpheus.toml` format error!"))
});
