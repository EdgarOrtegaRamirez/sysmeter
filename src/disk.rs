use std::fmt;
use sysinfo::Disks;

use crate::report::FormatReport;

/// Disk usage information
#[derive(Debug, Clone, serde::Serialize)]
pub struct DiskInfo {
    pub mount_point: String,
    pub name: String,
    pub file_system: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_bytes: u64,
    pub usage_percent: f64,
}

impl fmt::Display for DiskInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render_text())
    }
}

fn human_size(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.2} GiB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.2} MiB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.2} KiB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

impl FormatReport for DiskInfo {
    fn render_text(&self) -> String {
        format!(
            "{} ({}) — {}/{} ({:.1}%)",
            self.mount_point,
            self.name,
            human_size(self.used_bytes),
            human_size(self.total_bytes),
            self.usage_percent,
        )
    }
}

/// Collect disk usage information using sysinfo
pub fn get_disk_info() -> anyhow::Result<Vec<DiskInfo>> {
    let disks = Disks::new_with_refreshed_list();

    let infos: Vec<DiskInfo> = disks
        .iter()
        .map(|d| {
            let total = d.total_space();
            let available = d.available_space();
            let used = total.saturating_sub(available);
            let usage_pct = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            DiskInfo {
                mount_point: d.mount_point().display().to_string(),
                name: d.name().to_string_lossy().to_string(),
                file_system: d.file_system().to_string_lossy().to_string(),
                total_bytes: total,
                available_bytes: available,
                used_bytes: used,
                usage_percent: usage_pct,
            }
        })
        .collect();

    Ok(infos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_disk_info() {
        let disks = get_disk_info().unwrap();
        assert!(!disks.is_empty(), "Should detect at least one disk");
        // Root partition should exist
        let root = disks.iter().find(|d| d.mount_point == "/");
        assert!(root.is_some(), "Should have root mount point");
        if let Some(r) = root {
            assert!(r.total_bytes > 0, "Root disk should have > 0 total space");
            assert!(
                r.usage_percent >= 0.0 && r.usage_percent <= 100.0,
                "Usage percent should be within range"
            );
        }
    }

    #[test]
    fn test_disk_render_text() {
        let info = DiskInfo {
            mount_point: "/".to_string(),
            name: "nvme0n1p2".to_string(),
            file_system: "ext4".to_string(),
            total_bytes: 500_000_000_000,
            available_bytes: 200_000_000_000,
            used_bytes: 300_000_000_000,
            usage_percent: 60.0,
        };
        let text = info.render_text();
        assert!(text.contains("/"));
        assert!(text.contains("60.0"));
    }

    #[test]
    fn test_disk_render_json() {
        let info = DiskInfo {
            mount_point: "/".to_string(),
            name: "nvme0n1".to_string(),
            file_system: "ext4".to_string(),
            total_bytes: 500_000_000_000,
            available_bytes: 250_000_000_000,
            used_bytes: 250_000_000_000,
            usage_percent: 50.0,
        };
        let json = info.render_json();
        assert!(json.contains("\"mount_point\""));
        assert!(json.contains("\"/\""));
    }
}