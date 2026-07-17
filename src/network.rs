use std::fmt;
use sysinfo::Networks;

use crate::report::FormatReport;

/// Network interface throughput information
#[derive(Debug, Clone, serde::Serialize)]
pub struct NetworkInfo {
    pub interface_name: String,
    pub total_received_bytes: u64,
    pub total_transmitted_bytes: u64,
}

impl fmt::Display for NetworkInfo {
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

impl FormatReport for NetworkInfo {
    fn render_text(&self) -> String {
        format!(
            "{} — RX: {} | TX: {}",
            self.interface_name,
            human_size(self.total_received_bytes),
            human_size(self.total_transmitted_bytes),
        )
    }
}

/// Collect network I/O information using sysinfo
pub fn get_network_info() -> anyhow::Result<Vec<NetworkInfo>> {
    let networks = Networks::new_with_refreshed_list();

    let infos: Vec<NetworkInfo> = networks
        .iter()
        .map(|(name, data)| NetworkInfo {
            interface_name: name.clone(),
            total_received_bytes: data.total_received(),
            total_transmitted_bytes: data.total_transmitted(),
        })
        .filter(|n| n.total_received_bytes > 0 || n.total_transmitted_bytes > 0)
        .collect();

    Ok(infos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_network_info() {
        let networks = get_network_info().unwrap();
        // May be empty in container environments, but should not error
        if !networks.is_empty() {
            for net in &networks {
                assert!(
                    !net.interface_name.is_empty(),
                    "Interface name should not be empty"
                );
            }
        }
    }

    #[test]
    fn test_network_render_text() {
        let info = NetworkInfo {
            interface_name: "eth0".to_string(),
            total_received_bytes: 1_000_000_000,
            total_transmitted_bytes: 500_000_000,
        };
        let text = info.render_text();
        assert!(text.contains("eth0"));
        assert!(text.contains("RX"));
        assert!(text.contains("TX"));
    }

    #[test]
    fn test_network_render_json() {
        let info = NetworkInfo {
            interface_name: "lo".to_string(),
            total_received_bytes: 5000,
            total_transmitted_bytes: 6000,
        };
        let json = info.render_json();
        assert!(json.contains("\"interface_name\""));
        assert!(json.contains("\"lo\""));
    }
}
