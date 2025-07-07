use procfs::CpuInfo;
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
            // This is a placeholder for AMD GPU temperature reading
            // rocm-smi or other tools would be needed here.
            return None; // Not implemented for now
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
