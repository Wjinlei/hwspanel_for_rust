pub mod cli;
pub mod config;

use self::config::AppConfig;
use state::Storage;

pub static APP_CONTENT: Storage<AppConfig> = Storage::new();
