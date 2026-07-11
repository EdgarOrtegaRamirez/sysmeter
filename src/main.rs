use clap::{Parser, Subcommand};
use sysmeter::{
    cpu::get_cpu_info,
    disk::get_disk_info,
    memory::get_memory_info,
    network::get_network_info,
    process::get_top_processes,
    report::{FormatReport, OutputFormat},
};

#[derive(Parser)]
#[command(name = "sysmeter", version, about = "Lightweight system resource monitor CLI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show system summary (CPU, memory, disk, network)
    Summary {
        /// Output format
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },
    /// Show CPU usage
    Cpu {
        /// Output format
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },
    /// Show memory usage
    Memory {
        /// Output format
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },
    /// Show disk usage
    Disk {
        /// Output format
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },
    /// Show network I/O
    Network {
        /// Output format
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },
    /// Show top processes by CPU or memory
    Processes {
        /// Sort by (cpu, mem)
        #[arg(long, default_value = "cpu")]
        sort: String,
        /// Number of processes to show
        #[arg(long, default_value_t = 10)]
        top: usize,
        /// Output format
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },
    /// Show all metrics (same as `summary`)
    All {
        /// Output format
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(cmd) => match cmd {
            Commands::Summary { format } => {
                let report = collect_all()?;
                println!("{}", report.render(*format));
            }
            Commands::Cpu { format } => {
                let info = get_cpu_info()?;
                println!("{}", info.render(*format));
            }
            Commands::Memory { format } => {
                let info = get_memory_info()?;
                println!("{}", info.render(*format));
            }
            Commands::Disk { format } => {
                let infos = get_disk_info()?;
                for i in infos {
                    println!("{}", i.render(*format));
                }
            }
            Commands::Network { format } => {
                let infos = get_network_info()?;
                for i in infos {
                    println!("{}", i.render(*format));
                }
            }
            Commands::Processes { sort, top, format } => {
                let procs = get_top_processes(*top, sort)?;
                println!("{}", procs.render(*format));
            }
            Commands::All { format } => {
                let report = collect_all()?;
                println!("{}", report.render(*format));
            }
        },
        None => {
            // Default: show summary in text
            let report = collect_all()?;
            println!("{}", report.render(OutputFormat::Text));
        }
    }

    Ok(())
}

fn collect_all() -> anyhow::Result<SystemReport> {
    let cpu = get_cpu_info()?;
    let memory = get_memory_info()?;
    let disks = get_disk_info()?;
    let networks = get_network_info()?;
    let processes = get_top_processes(5, "cpu")?;
    Ok(SystemReport { cpu, memory, disks, networks, processes })
}

#[derive(serde::Serialize)]
pub struct SystemReport {
    pub cpu: sysmeter::cpu::CpuInfo,
    pub memory: sysmeter::memory::MemoryInfo,
    pub disks: Vec<sysmeter::disk::DiskInfo>,
    pub networks: Vec<sysmeter::network::NetworkInfo>,
    pub processes: sysmeter::process::ProcessList,
}

impl FormatReport for SystemReport {
    fn render_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("{}\n", self.cpu.render_text()));
        out.push_str(&format!("{}\n", self.memory.render_text()));
        out.push_str("── Disks ──\n");
        for d in &self.disks {
            out.push_str(&format!("  {}\n", d.render_text()));
        }
        out.push_str("\n── Network ──\n");
        for n in &self.networks {
            out.push_str(&format!("  {}\n", n.render_text()));
        }
        out.push_str(&format!("\n── Top Processes ──\n{}", self.processes.render_text()));
        out
    }
}