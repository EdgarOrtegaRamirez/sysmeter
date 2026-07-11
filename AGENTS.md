# AGENTS.md — For AI Agents

## Repository Purpose

**sysmeter** is a lightweight system resource monitor CLI tool built in Rust. It provides real-time readings of CPU, memory, disk, and network usage, plus a view of top processes sorted by CPU or memory consumption.

## Key Technical Details

- **Language**: Rust (edition 2021)
- **Build system**: Cargo
- **Dependencies**: clap 4.6 (derive), sysinfo 0.34, serde/serde_json 1.0, anyhow 1.0
- **CLI framework**: clap with subcommands
- **Output formats**: Text (default) and JSON (`--format json`)
- **System data source**: sysinfo crate (reads from `/proc` on Linux, sysctl on macOS, etc.)

## Entry Points

- **Binary entry**: `src/main.rs` — parses CLI args and dispatches to subsystem modules
- **Library entry**: `src/lib.rs` — re-exports all public modules
- **Tests**: `cargo test` runs unit tests embedded in each module (18 tests total)

## Testing

```bash
cargo test
```

## Build & Run

```bash
cargo build --release
./target/release/sysmeter
./target/release/sysmeter cpu --format json
./target/release/sysmeter processes --sort mem --top 5
```

## Architecture Pattern

Each subsystem module follows:
1. A data struct (`CpuInfo`, `MemoryInfo`, `DiskInfo`, `NetworkInfo`, `ProcessList`) implementing `Serialize`
2. A `FormatReport` implementation (from `report.rs`) for text/JSON rendering
3. A `get_*_info()` public function that samples sysinfo data

## NaN Platform API Integration

This project currently does **not** use the NaN Builders API. It is a standalone CLI tool with no AI dependencies.

## Security

- No hardcoded secrets
- No network calls to external services
- Reads system stats from the OS via sysinfo
- All file operations use safe Rust abstractions

## GitHub

- Owner: EdgarOrtegaRamirez
- Repo: https://github.com/EdgarOrtegaRamirez/sysmeter
- License: MIT