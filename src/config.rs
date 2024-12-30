use serde::Deserialize;

/// Rust representation of the TOML config for Orpheus.
/// Example can be found in `orpheus-EXAMPLE.toml`.
#[derive(Deserialize)]
pub struct Config {
    server: ServerConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    data_path: String,
}
