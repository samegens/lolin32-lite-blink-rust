# Blinking LED on Lolin32 Lite using Rust

## Setup

```bash
cargo install espup
espup install
rustup toolchain list
# Should display 'esp'.

# Install requirements for esp-generate:
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
cargo install espflash --locked
cargo install esp-config --features=tui --locked

# Install esp-generate itself:
cargo install esp-generate --locked
```

## Create project

```bash
esp-generate --chip esp32 --headless lolin32-lite-blink-rust
cd lolin32-blink-rust

# Don't forget this (needed to use the correct tools for ESP32 builds):
. ~/export-esp.sh
```

## Improve upload speed

In .cargo/config.toml, change the runner line to:

```ini
runner = "espflash flash --monitor --chip esp32 --baud 921600"
```

This will reduce upload time.

## Build and run project

```bash
cargo run
```

This will build and upload the binary and display the output when running.

## What if I have a regular Lolin32?

Then change the port to GPIO5.
