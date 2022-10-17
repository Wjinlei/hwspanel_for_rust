#[derive(serde::Serialize, serde::Deserialize, Getters, Setters, PartialEq, Debug, Clone)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct ServerConfig {
    host: String,
    port: String,
}

#[derive(serde::Serialize, serde::Deserialize, Getters, Setters, PartialEq, Debug, Clone)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct AppConfig {
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
