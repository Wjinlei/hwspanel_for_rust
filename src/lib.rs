pub mod api;
pub mod config;

#[macro_use]
extern crate getset;

use config::config::AppConfig;
use tokio::fs::read_to_string;

// Load config
pub async fn load_config() -> AppConfig {
    let config_data = read_to_string("application.yaml").await.unwrap();
    let config = AppConfig::new(config_data.as_str());
    config
}
