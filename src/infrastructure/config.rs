use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use tokio::fs::read_to_string;

#[derive(Serialize, Deserialize, Getters, Setters, PartialEq, Debug, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct ServerConfig {
    host: String,
    port: String,
}

#[derive(Serialize, Deserialize, Getters, Setters, PartialEq, Debug, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct AppConfig {
    debug: bool,
    server: ServerConfig,
}

impl AppConfig {
    pub fn new(yaml_data: &str) -> Self {
        let config = match serde_yaml::from_str(yaml_data) {
            Ok(conf) => conf,
            Err(err) => panic!("{}", err),
        };
        config
    }
}

// Load config
pub async fn load_config() -> AppConfig {
    let config_data = read_to_string("application.yaml").await.unwrap();
    let config = AppConfig::new(config_data.as_str());
    config
}
