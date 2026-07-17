use std::fmt;
use sysinfo::System;

use crate::report::FormatReport;

/// CPU usage information
#[derive(Debug, Clone, serde::Serialize)]
pub struct CpuInfo {
    pub name: String,
    pub brand: String,
    pub vendor_id: String,
    pub cores: usize,
    pub usage_percent: f64,
}

impl fmt::Display for CpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render_text())
    }
}

impl FormatReport for CpuInfo {
    fn render_text(&self) -> String {
        format!(
            "┌─ CPU ──────────────────────\n\
             │ Model:     {} {}\n\
             │ Cores:     {}\n\
             │ Usage:     {:.1}%\n\
             └───────────────────────────",
            self.brand, self.name, self.cores, self.usage_percent
        )
    }
}

/// Collect CPU information.
/// Uses sysinfo to get a snapshot of CPU usage.
pub fn get_cpu_info() -> anyhow::Result<CpuInfo> {
    let mut system = System::new_all();
    std::thread::sleep(std::time::Duration::from_millis(200));
    system.refresh_cpu_usage();

    let cpus = system.cpus();
    let global_usage = system.global_cpu_usage();

    let name = cpus
        .first()
        .map(|c| c.name().to_string())
        .unwrap_or_default();
    let brand = cpus
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_default();
    let vendor_id = cpus
        .first()
        .map(|c| c.vendor_id().to_string())
        .unwrap_or_default();

    Ok(CpuInfo {
        name,
        brand,
        vendor_id,
        cores: cpus.len(),
        usage_percent: global_usage as f64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cpu_info() {
        let info = get_cpu_info().unwrap();
        assert!(info.cores > 0, "Should detect at least 1 CPU core");
        assert!(info.usage_percent >= 0.0, "CPU usage should be >= 0%");
        assert!(info.usage_percent <= 100.0, "CPU usage should be <= 100%");
    }

    #[test]
    fn test_cpu_render_text() {
        let info = CpuInfo {
            name: "CPU".to_string(),
            brand: "Intel".to_string(),
            vendor_id: "GenuineIntel".to_string(),
            cores: 8,
            usage_percent: 42.5,
        };
        let text = info.render_text();
        assert!(text.contains("Intel"));
        assert!(text.contains("8"));
        assert!(text.contains("42.5"));
    }

    #[test]
    fn test_cpu_render_json() {
        let info = CpuInfo {
            name: "TestCPU".to_string(),
            brand: "TestBrand".to_string(),
            vendor_id: "GenuineTest".to_string(),
            cores: 4,
            usage_percent: 50.0,
        };
        let json = info.render_json();
        assert!(json.contains("\"usage_percent\""));
        assert!(json.contains("50.0"));
    }
}
