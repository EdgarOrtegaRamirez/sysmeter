# sysmeter

**sysmeter** is a lightweight system resource monitor CLI written in Rust. It provides real-time insight into CPU, memory, disk, and network usage, plus a view of the top processes by CPU or memory consumption.

## Features

- **CPU** — model, cores, and usage percentage
- **Memory** — RAM and swap usage with human-readable sizes
- **Disk** — per-mount-point capacity, usage, and percentage
- **Network** — per-interface total received/transmitted bytes
- **Processes** — top N processes sorted by CPU or memory
- **Output formats** — text (default) or JSON (`--format json`)

## Installation

### From source

```bash
git clone https://github.com/EdgarOrtegaRamirez/sysmeter.git
cd sysmeter
cargo build --release
# Binary at ./target/release/sysmeter
```

### Using Cargo

```bash
cargo install --git https://github.com/EdgarOrtegaRamirez/sysmeter.git
```

## Usage

```bash
# Show all metrics (default)
sysmeter

# Show only CPU info
sysmeter cpu

# Show memory info as JSON
sysmeter memory --format json

# Show disk usage
sysmeter disk

# Show network I/O
sysmeter network

# Show top 10 processes by CPU (default)
sysmeter processes

# Show top 5 processes by memory
sysmeter processes --sort mem --top 5

# Show everything in JSON
sysmeter all --format json

# Same as `all` (alias)
sysmeter summary
```

### Output examples

#### Text format (default)

```
┌─ CPU ──────────────────────
│ Model:     Intel(R) Xeon(R) Gold 5412U CPU
│ Cores:     2
│ Usage:     8.5%
└───────────────────────────
┌─ Memory ───────────────────
│ RAM:    1.22 GiB/3.93 GiB (31.1%)
│ Swap:   0 B/0 B (0.0%)
└───────────────────────────
── Disks ──
  / — 189.81 GiB/877.64 GiB (21.6%)
── Network ──
  eth0 — RX: 7.68 GiB | TX: 7.87 GiB
── Top Processes ──
     PID    CPU%      MEM%         MEM  NAME
──────────────────────────────────────────────────
  123456  4.3   %     0.1%    5.62 MiB  sysmeter
```

#### JSON format

```json
{
  "name": "cpu0",
  "brand": "Intel(R) Xeon(R) Gold 5412U",
  "vendor_id": "GenuineIntel",
  "cores": 2,
  "usage_percent": 6.67
}
```

## Subcommands

| Subcommand  | Description                            | Default format |
|-------------|----------------------------------------|----------------|
| `cpu`       | CPU model, cores, and usage            | text           |
| `memory`    | RAM and swap usage                     | text           |
| `disk`      | Disk usage per mount point             | text           |
| `network`   | Network I/O per interface              | text           |
| `processes` | Top N processes                        | text           |
| `all`       | All metrics (same as no subcommand)    | text           |
| `summary`   | Alias for `all`                        | text           |

All subcommands accept `--format [text|json]`.

## Architecture

```
src/
├── main.rs        — CLI entry point (clap parser, dispatch)
├── lib.rs         — Module re-exports
├── cpu.rs         — CPU info collection and formatting
├── memory.rs      — Memory info collection and formatting
├── disk.rs        — Disk info collection and formatting
├── network.rs     — Network I/O collection and formatting
├── process.rs     — Process listing with sorting
└── report.rs      — OutputFormat enum and FormatReport trait
```

The library crate (`sysmeter`) exposes public modules so the CLI binary (`main.rs`) and tests can both consume them. Each subsystem follows the same pattern: a data struct, a `FormatReport` impl, and a `get_*` function that samples sysinfo data.

## Dependencies

| Crate      | Version | Purpose                        |
|------------|---------|--------------------------------|
| `clap`     | 4.6     | CLI argument parsing (derive)  |
| `sysinfo`  | 0.34    | Cross-platform system stats    |
| `serde`    | 1.0     | JSON serialization             |
| `serde_json` | 1.0   | JSON output                    |
| `anyhow`   | 1.0     | Error handling                 |

## Testing

```bash
cargo test
```

## Security

No secrets, no network calls to external services, no privilege escalation. The tool reads `/proc` (on Linux) via the sysinfo library for all system stats. See [SECURITY.md](./SECURITY.md) for details.

## License

MIT — see [LICENSE](./LICENSE).