use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::from_utf8};
use sysinfo::{CpuExt, DiskExt, NetworkExt, SystemExt};

#[derive(Serialize, Deserialize, Debug)]
pub struct DiskDTO {
    pub name: String,         // "/dev/sda1"
    pub file_system: String,  // "vfat"
    pub mount_point: PathBuf, // "boot"
    pub total_space: u64,     // 535801856
    pub available_space: u64, // 426647552
    pub is_removable: bool,   // false
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkDTO {
    pub name: String, // "enp2s0"
    pub received: u64,
    pub total_received: u64,
    pub transmitted: u64,
    pub total_transmitted: u64,
    pub packets_received: u64,
    pub total_packets_received: u64,
    pub packets_transmitted: u64,
    pub total_packets_transmitted: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CPUDTO {
    pub name: String,
    pub frequency: u64,
    pub cpu_usage: f32,
    pub vendor_id: String,
    pub brand: String,
}

/// Get Disk Information
pub fn get_disks() -> Vec<DiskDTO> {
    let mut sys = super::SYS_INFO.get().write().unwrap();
    sys.refresh_disks_list();
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

/// Get Network Information
pub fn get_networks() -> Vec<NetworkDTO> {
    let mut sys = super::SYS_INFO.get().write().unwrap();
    sys.refresh_networks_list();
    sys.networks()
        .into_iter()
        .map(|network| NetworkDTO {
            name: network.0.to_string(),
            received: network.1.received(),
            total_received: network.1.total_received(),
            transmitted: network.1.transmitted(),
            total_transmitted: network.1.total_transmitted(),
            packets_received: network.1.packets_received(),
            total_packets_received: network.1.total_packets_received(),
            packets_transmitted: network.1.packets_transmitted(),
            total_packets_transmitted: network.1.total_packets_transmitted(),
        })
        .collect()
}

/// Get CPU Information
pub fn get_cpus() -> Vec<CPUDTO> {
    let mut sys = super::SYS_INFO.get().write().unwrap();
    sys.refresh_cpu();
    sys.cpus()
        .into_iter()
        .map(|cpu| CPUDTO {
            name: cpu.name().to_string(),
            frequency: cpu.frequency(),
            cpu_usage: cpu.cpu_usage(),
            vendor_id: cpu.vendor_id().to_string(),
            brand: cpu.brand().to_string(),
        })
        .collect()
}
