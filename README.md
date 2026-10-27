# RNV (Ruby and Rust Nano Variant)

RNV is a hybrid development repository hosting standalone implementations of a minimalist terminal text editor styled exactly like GNU Nano. It includes an instant single-file monolithic engine built on Ruby and a high-performance compiled core engine built on Rust.

The repository also includes utility addons for text statistics and background directory maintenance.

## Project Structure

rnv-editor/
├── rnv            # Monolithic Ruby implementation (No external dependencies)
├── analyzer.rb    # Ruby text statistic and metrics analyzer script
├── cleaner.exs    # Elixir background backup and log cleaner tool
├── Cargo.toml     # Rust build manifest
└── src/           # High-performance Compiled Rust codebase
    ├── main.rs    # Event capture loops and CLI arguments
    ├── core.rs    # Memory grid buffer modifiers
    └── ui.rs      # Optimized double-buffered pink screen flushers

## Unified Controls

- Direct Type: Write words immediately into the document canvas.
- Ctrl + O: Triggers the interactive Save Prompt at the bottom row.
- Ctrl + X: Purge memory and gracefully exit to your shell session.
- Arrow Keys: Fluid grid navigation across character limits.

## Execution and Deployment

### Run using the Ruby Engine
```bash
chmod +x rnv
./rnv document.txt
```

### Run using the Compiled Rust Engine
Ensure you have the stable toolchain installed (sudo pacman -S rust):
```bash
cargo build --release
./target/release/rnv document.txt
```

### Run Addons
- Text Analyzer (Ruby):
```bash
ruby analyzer.rb document.txt
```
- System Cleaner (Elixir):
```bash
elixir cleaner.exs
```
