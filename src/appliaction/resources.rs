use std::{path::PathBuf, str::from_utf8};

use serde::{Deserialize, Serialize};
use sysinfo::{DiskExt, RefreshKind, System, SystemExt};

#[derive(Serialize, Deserialize, Debug)]
pub struct DiskDTO {
    pub name: String,         // "/dev/sda1"
    pub file_system: String,  // "vfat"
    pub mount_point: PathBuf, // "boot"
    pub total_space: u64,     // 535801856
    pub available_space: u64, // 426647552
    pub is_removable: bool,   // false
}

/// Get Disk Information
///
/// # Examples
///
/// ```
/// for disk in get_disks {
///     println!("{:?}", disk);
/// }
/// ```
pub fn get_disks() -> Vec<DiskDTO> {
    System::new_with_specifics(RefreshKind::new().with_disks_list())
        .disks()
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
