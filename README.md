# Blinking LED on Lolin32 Lite using Rust

This project shows a 'hello world' (ie. blinking LED) program on a Lolin32 Lite compiled in Rust.
It was created with esp-generate and adjusted to be a more developer-friendly base for other projects:
println and stack traces where added.

## Setup (on Linux)

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

## Modify the generated project

I added the esp-println and esp-backtrace crates to make debugging easier and added code to main.rs to make the internal LED blink.

## Build and run

Connect the Lolin32 Lite to a USB port.

```bash
cargo run
```

Toolset will do the rest automatically.

## Improve upload speed

In .cargo/config.toml, change the runner line to:

```ini
runner = "espflash flash --monitor --chip esp32 --baud 921600"
```

This will reduce upload time by about a factor four. If upload fails, remove the increased baud rate.

## Build and run project

```bash
cargo run
```

This will build and upload the binary and display the output when running.

## What if I have a regular Lolin32?

Then change the port to GPIO5. It should work then, but I have only a Lite version, so I can't test it.
