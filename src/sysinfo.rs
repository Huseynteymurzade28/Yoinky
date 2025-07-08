use procfs::{CpuInfo, Meminfo};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum GpuType {
    Nvidia,
    Amd,
    Unknown,
}

impl fmt::Display for GpuType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn cpu_core_count() -> usize {
    CpuInfo::new().unwrap().num_cores()
}

pub fn cpu_temp() -> Option<f32> {
    let path = "/sys/class/thermal/thermal_zone0/temp";
    let temp = std::fs::read_to_string(path).ok()?;
    let value = temp.trim().parse::<f32>().ok()?;
    Some(value / 1000.0)
}

pub fn gpu_info() -> Option<(GpuType, i32)> {
    match get_gpu_type() {
        GpuType::Nvidia => {
            let output = std::process::Command::new("nvidia-smi")
                .arg("--query-gpu=temperature.gpu")
                .arg("--format=csv,noheader,nounits")
                .output()
                .ok()?;
            if output.status.success() {
                let temp_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if let Ok(temp) = temp_str.parse::<i32>() {
                    return Some((GpuType::Nvidia, temp));
                }
            }
        }
        GpuType::Amd => {
            let path = "/sys/class/drm/card0/device/hwmon/hwmon0/temp1_input";
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(temp) = content.trim().parse::<i32>() {
                    return Some((GpuType::Amd, temp / 1000)); // Convert to Celsius
                }
            }
        }
        GpuType::Unknown => {
            return None;
        }
    }
    None
}

fn get_gpu_type() -> GpuType {
    if std::path::Path::new("/proc/driver/nvidia/version").exists() {
        GpuType::Nvidia
    } else if std::path::Path::new("/sys/class/drm/card0/device/vendor").is_file() {
        if let Ok(vendor) = std::fs::read_to_string("/sys/class/drm/card0/device/vendor") {
            if vendor.trim() == "0x1002" {
                // AMD's PCI vendor ID
                return GpuType::Amd;
            }
        }
        GpuType::Unknown
    } else {
        GpuType::Unknown
    }
}

pub fn cpu_usage() -> Option<f32> {
    let path = "/proc/stat";
    let content = std::fs::read_to_string(path).ok()?;
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return None;
    }

    let cpu_line = lines[0];
    let values: Vec<&str> = cpu_line.split_whitespace().collect();
    if values.len() < 5 {
        return None;
    }

    let user: f32 = values[1].parse().ok()?;
    let nice: f32 = values[2].parse().ok()?;
    let system: f32 = values[3].parse().ok()?;
    let idle: f32 = values[4].parse().ok()?;

    let total = user + nice + system + idle;
    Some((total - idle) / total * 100.0)
}

/// Returns RAM usage as a tuple of (used_mb, total_mb)
pub fn ram_usage() -> Option<(u64, u64)> {
    let meminfo = Meminfo::new().ok()?;
    let total = meminfo.mem_total / 1024; // Convert from KB to MB
                                          // MemAvailable is a more accurate measure of "free" memory than MemFree
    let available = meminfo.mem_available.unwrap_or(meminfo.mem_free) / 1024;
    let used = total - available;
    Some((used, total))
}

/// Returns total disk usage as a tuple of (used_gb, total_gb)
pub fn disk_usage() -> Option<(f32, f32)> {
    // Using `df --output=used,size --total` provides machine-readable numbers in KB.
    let output = std::process::Command::new("df")
        .arg("--total")
        .arg("--output=used,size")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    // The last line of the output is the total.
    if let Some(line) = output_str.lines().last() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }
        // The output is in KB, parse and convert to GB.
        let used_kb: f32 = parts[0].parse().ok()?;
        let total_kb: f32 = parts[1].parse().ok()?;
        return Some((used_kb / 1024.0 / 1024.0, total_kb / 1024.0 / 1024.0));
    }
    None
}
