# Rusty Dice

A basic Hello World Rust application using MacroQuad for the UI.

## Features

- Simple "Hello World" text display
- Visual dice representation with dots
- Cross-platform graphics using MacroQuad

## Requirements

- Rust (latest stable version)
- Cargo

## Running the Application

1. Make sure you have Rust installed on your system
2. Clone or download this repository
3. Navigate to the project directory
4. Run the application:

```bash
cargo run
```

## Building for Release

To create an optimized release build:

```bash
cargo build --release
```

The executable will be created in `target/release/rusty_dice`.

## Project Structure

- `src/main.rs` - Main application code
- `Cargo.toml` - Project dependencies and configuration

## Dependencies

- `macroquad` - Cross-platform game framework for Rust
