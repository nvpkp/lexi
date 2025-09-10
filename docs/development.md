# 🛠️ Development

Guide for contributing to the Lexi compiler.

## Setup

```bash
# Clone and build
git clone https://github.com/yourusername/lexi.git
cd lexi
cargo build

# Run tests
cargo test

# Run compiler
cargo run -- compile examples/fibonacci.lxi
```

## Project Structure

```
src/
├── main.rs           # CLI interface
├── compiler.rs       # Core compilation logic  
├── config.rs         # Configuration management
└── providers/        # AI provider implementations
    ├── openai.rs
    ├── anthropic.rs
    └── ollama.rs
```

## Building

### Development
```bash
cargo build                    # Debug build
cargo run -- compile test.lxi # Run directly
```

### Release
```bash
cargo build --release         # Optimized binary
```

### Cross-Platform
```bash
# Install targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin

# Build for platforms
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-apple-darwin
```

## Testing

```bash
cargo test                     # All tests
cargo test --test integration  # Integration tests only
```

## Code Quality

```bash
cargo fmt                      # Format code
cargo clippy                   # Lint code
```

## Adding Features

### New AI Provider

1. Create `src/providers/newprovider.rs`
2. Add to compiler match statement
3. Update configuration options
4. Add tests and documentation

### New Target Language

1. Add file extension mapping
2. Update code generation prompts
3. Add execution support for `--run`
4. Test with examples

## Debugging

```bash
RUST_LOG=debug cargo run -- compile test.lxi
```

## Release Process

1. Update version in `Cargo.toml`
2. Run tests: `cargo test`
3. Build release binaries for all platforms
4. Create GitHub release with binaries
5. Update documentation
