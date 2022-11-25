use std::{path::PathBuf, str::from_utf8};

use serde::{Deserialize, Serialize};
use sysinfo::{DiskExt, System, SystemExt};

#[derive(Serialize, Deserialize, Debug)]
pub struct DiskDTO {
    pub name: String,
    pub file_system: String,
    pub mount_point: PathBuf,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
}

pub fn get_disks() -> Vec<DiskDTO> {
    let mut sys = System::new_all();
    sys.refresh_disks();
    sys.disks()
        .into_iter()
        .map(|disk| DiskDTO {
            name: match disk.name().to_str() {
                Some(str) => str.to_string(),
                None => "Unknown".to_string(),
            },
            file_system: match from_utf8(disk.file_system()) {
                Ok(str) => str.to_string(),
                Err(_) => "Unknown".to_string(),
            },
            mount_point: disk.mount_point().to_path_buf(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            is_removable: disk.is_removable(),
        })
        .collect()
}
