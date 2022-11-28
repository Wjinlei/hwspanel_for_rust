pub mod resources;

use state::Storage;
use std::sync::RwLock;
use sysinfo::System;

pub static SYS_INFO: Storage<RwLock<System>> = Storage::new();
