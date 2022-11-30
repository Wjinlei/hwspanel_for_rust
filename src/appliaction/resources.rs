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

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemDTO {
    pub name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub host_name: Option<String>,
    pub load_avg: LoadAvgDTO,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadAvgDTO {
    /// Average load within one minute.
    pub one: f64,
    /// Average load within five minutes.
    pub five: f64,
    /// Average load within fifteen minutes.
    pub fifteen: f64,
    // 计算平均负载 https://www.tecmint.com/understand-linux-load-averages-and-monitor-performance/
    pub percent: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryDTO {
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64, // 空闲内存 https://docs.rs/sysinfo/0.26.8/sysinfo/trait.SystemExt.html#tymethod.free_memory
    pub available_memory: u64, // 可用内存 https://docs.rs/sysinfo/0.26.8/sysinfo/trait.SystemExt.html#tymethod.available_memory
    pub percent_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub free_swap: u64,
    pub percent_swap: u64,
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

/// Get System Information
pub fn get_system() -> SystemDTO {
    let sys = super::SYS_INFO.get().write().unwrap();
    let load_avg = sys.load_average();

    // 计算平均负载 https://www.tecmint.com/understand-linux-load-averages-and-monitor-performance/
    let max = sys.cpus().len() * 2; // *2 是为了考虑超线程的情况，当超线程开启后，逻辑 CPU 的个数是核数的两倍
    let percent = (load_avg.one * 100f64) as usize / max;
    let percent = if percent > 100 { 100 } else { percent };

    SystemDTO {
        name: sys.name(),
        kernel_version: sys.kernel_version(),
        os_version: sys.os_version(),
        host_name: sys.host_name(),
        load_avg: LoadAvgDTO {
            percent,
            one: load_avg.one,
            five: load_avg.five,
            fifteen: load_avg.fifteen,
        },
    }
}

/// Get Memory Information
pub fn get_memory() -> MemoryDTO {
    let mut sys = super::SYS_INFO.get().write().unwrap();
    sys.refresh_memory();
    MemoryDTO {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        free_memory: sys.free_memory(),
        available_memory: sys.available_memory(),
        percent_memory: ((sys.used_memory() as f64 / sys.total_memory() as f64) * 100f64) as u64,
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
        free_swap: sys.free_swap(),
        percent_swap: ((sys.used_swap() as f64 / sys.total_swap() as f64) * 100f64) as u64,
    }
}
