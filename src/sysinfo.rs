use procfs::CpuInfo;

pub fn cpu_core_count() -> usize {
    CpuInfo::new().unwrap().num_cores()
}

pub fn cpu_temp() -> Option<f32> {
    let path = "/sys/class/thermal/thermal_zone0/temp";
    let temp = std::fs::read_to_string(path).ok()?;
    let value = temp.trim().parse::<f32>().ok()?;
    Some(value / 1000.0)
}

pub fn gpu_temp() -> Option<i32> {
    let nvdia_smi_path = "/usr/bin/nvidia-smi";
    if std::path::Path::new(nvdia_smi_path).exists() {
        let output = std::process::Command::new(nvdia_smi_path)
            .arg("--query-gpu=temperature.gpu")
            .arg("--format=csv,noheader,nounits")
            .output()
            .ok()?;
        if output.status.success() {
            let temp_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(temp) = temp_str.parse::<i32>() {
                return Some(temp);
    }
        }
    }
    None
}