# Security Policy

## Reporting a Vulnerability

This is a CLI system monitoring tool. It does **not**:

- Communicate over a network (no HTTP calls, no remote APIs)
- Accept or process untrusted input files
- Run as a privileged process (no `sudo` required for normal operation)
- Store or transmit user data

## What sysmeter accesses

sysmeter reads system statistics through the **sysinfo** library, which on Linux reads from `/proc` and `/sys` filesystem interfaces. These are the same files read by standard tools like `top`, `free`, and `df`.

## Best practices

- Run as an unprivileged user — sysmeter does not require root
- Pin the version in your Cargo.toml or CI pipeline — use exact versions, not `*`
- The tool has no network attack surface — no ports, no sockets, no remote calls

If you discover a security issue in sysinfo (the underlying dependency), please report it upstream at https://github.com/GuillaumeGomez/sysinfo

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 0.1.x   | ✅ Active          |