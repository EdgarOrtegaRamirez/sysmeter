use std::fmt;
use sysinfo::System;

use crate::report::FormatReport;

/// Memory usage information
#[derive(Debug, Clone, serde::Serialize)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub usage_percent: f64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_usage_percent: f64,
}

impl fmt::Display for MemoryInfo {
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

impl FormatReport for MemoryInfo {
    fn render_text(&self) -> String {
        format!(
            "┌─ Memory ───────────────────\n\
             │ RAM:    {}/{} ({:.1}%)\n\
             │ Swap:   {}/{} ({:.1}%)\n\
             └───────────────────────────",
            human_size(self.used_bytes),
            human_size(self.total_bytes),
            self.usage_percent,
            human_size(self.swap_used_bytes),
            human_size(self.swap_total_bytes),
            self.swap_usage_percent,
        )
    }
}

/// Collect memory information using sysinfo
pub fn get_memory_info() -> anyhow::Result<MemoryInfo> {
    let mut system = System::new_all();
    system.refresh_memory();

    let total = system.total_memory();
    let used = system.used_memory();
    let free = system.free_memory();
    let swap_total = system.total_swap();
    let swap_used = system.used_swap();

    let usage_percent = if total > 0 {
        (used as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let swap_usage_percent = if swap_total > 0 {
        (swap_used as f64 / swap_total as f64) * 100.0
    } else {
        0.0
    };

    Ok(MemoryInfo {
        total_bytes: total,
        used_bytes: used,
        free_bytes: free,
        usage_percent,
        swap_total_bytes: swap_total,
        swap_used_bytes: swap_used,
        swap_usage_percent,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_memory_info() {
        let info = get_memory_info().unwrap();
        assert!(info.total_bytes > 0, "Total memory should be > 0");
        assert!(
            info.usage_percent >= 0.0 && info.usage_percent <= 100.0,
            "Memory usage percent out of range"
        );
    }

    #[test]
    fn test_memory_render_text() {
        let info = MemoryInfo {
            total_bytes: 16_000_000_000,
            used_bytes: 8_000_000_000,
            free_bytes: 8_000_000_000,
            usage_percent: 50.0,
            swap_total_bytes: 2_000_000_000,
            swap_used_bytes: 500_000_000,
            swap_usage_percent: 25.0,
        };
        let text = info.render_text();
        assert!(text.contains("RAM"));
        assert!(text.contains("50.0"));
        assert!(text.contains("25.0"));
    }

    #[test]
    fn test_memory_render_json() {
        let info = MemoryInfo {
            total_bytes: 8_000_000_000,
            used_bytes: 4_000_000_000,
            free_bytes: 4_000_000_000,
            usage_percent: 50.0,
            swap_total_bytes: 1_000_000_000,
            swap_used_bytes: 0,
            swap_usage_percent: 0.0,
        };
        let json = info.render_json();
        assert!(json.contains("\"usage_percent\""));
        assert!(json.contains("50.0"));
    }

    #[test]
    fn test_human_size() {
        assert_eq!(human_size(0), "0 B");
        assert_eq!(human_size(500), "500 B");
        assert_eq!(human_size(1024), "1.00 KiB");
        assert_eq!(human_size(1_048_576), "1.00 MiB");
        assert_eq!(human_size(1_073_741_824), "1.00 GiB");
    }
}
