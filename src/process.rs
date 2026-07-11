use std::fmt;
use sysinfo::System;

use crate::report::FormatReport;

/// Process information
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub memory_percent: f64,
    pub status: String,
}

/// Top processes list
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessList {
    pub processes: Vec<ProcessInfo>,
    pub sort_by: String,
    pub count: usize,
}

impl fmt::Display for ProcessInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:>8}  {:<6.1}%  {:>8.1}%  {:>10}  {}",
            self.pid,
            self.cpu_usage,
            self.memory_percent,
            human_size(self.memory_bytes),
            self.name,
        )
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

impl FormatReport for ProcessList {
    fn render_text(&self) -> String {
        let mut out = format!(
            "{:>8}  {:>6}  {:>8}  {:>10}  {}\n",
            "PID", "CPU%", "MEM%", "MEM", "NAME",
        );
        out.push_str(&"─".repeat(50));
        out.push('\n');
        for p in &self.processes {
            out.push_str(&format!("{}\n", p));
        }
        out
    }
}

/// Collect top processes by CPU or memory usage
pub fn get_top_processes(top: usize, sort_by: &str) -> anyhow::Result<ProcessList> {
    let mut system = System::new_all();
    std::thread::sleep(std::time::Duration::from_millis(200));
    system.refresh_all();

    let total_mem = system.total_memory();

    let processes: Vec<ProcessInfo> = system
        .processes()
        .iter()
        .map(|(pid, process)| {
            let mem = process.memory();
            let mem_pct = if total_mem > 0 {
                (mem as f64 / total_mem as f64) * 100.0
            } else {
                0.0
            };
            ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string_lossy().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_bytes: mem,
                memory_percent: mem_pct,
                status: format!("{:?}", process.status()),
            }
        })
        .collect();

    let mut sorted = match sort_by {
        "mem" | "memory" => {
            let mut p = processes;
            p.sort_by(|a, b| b.memory_bytes.cmp(&a.memory_bytes));
            p
        }
        _ => {
            let mut p = processes;
            p.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
            p
        }
    };

    sorted.truncate(top);

    Ok(ProcessList {
        processes: sorted,
        sort_by: sort_by.to_string(),
        count: top,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_top_processes_cpu() {
        let list = get_top_processes(5, "cpu").unwrap();
        assert_eq!(list.sort_by, "cpu");
        assert!(list.processes.len() <= 5, "Should limit to 5 processes");
        // Processes should be sorted by CPU descending (f32)
        for i in 1..list.processes.len() {
            assert!(
                list.processes[i - 1].cpu_usage >= list.processes[i].cpu_usage,
                "Processes should be sorted by CPU descending"
            );
        }
    }

    #[test]
    fn test_get_top_processes_memory() {
        let list = get_top_processes(3, "mem").unwrap();
        assert_eq!(list.count, 3);
        assert!(list.processes.len() <= 3, "Should limit to 3 processes");
        // Processes should be sorted by memory descending
        for i in 1..list.processes.len() {
            assert!(
                list.processes[i - 1].memory_bytes >= list.processes[i].memory_bytes,
                "Processes should be sorted by memory descending"
            );
        }
    }

    #[test]
    fn test_process_list_render_text() {
        let list = ProcessList {
            processes: vec![
                ProcessInfo {
                    pid: 1234,
                    name: "test-process".to_string(),
                    cpu_usage: 5.0,
                    memory_bytes: 10_000_000,
                    memory_percent: 1.5,
                    status: "Running".to_string(),
                },
            ],
            sort_by: "cpu".to_string(),
            count: 1,
        };
        let text = list.render_text();
        assert!(text.contains("PID"));
        assert!(text.contains("1234"));
        assert!(text.contains("test-process"));
        assert!(text.contains("CPU%"));
    }

    #[test]
    fn test_process_list_render_json() {
        let list = ProcessList {
            processes: vec![
                ProcessInfo {
                    pid: 5678,
                    name: "bash".to_string(),
                    cpu_usage: 0.1,
                    memory_bytes: 5_000_000,
                    memory_percent: 0.5,
                    status: "Sleep".to_string(),
                },
            ],
            sort_by: "cpu".to_string(),
            count: 1,
        };
        let json = list.render_json();
        assert!(json.contains("\"pid\""));
        assert!(json.contains("5678"));
    }
}