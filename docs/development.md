# 🛠️ Development

Guide for contributing to the Lexi compiler.

## Setup

```bash
# Clone and build
git clone https://github.com/nvpkp/lexi.git
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

### Cross-Platform Builds

**Quick build all platforms:**
```bash
# Linux/macOS
chmod +x build.sh
./build.sh

# Windows  
build.bat

# Specify version
./build.sh 1.2.0
```

**Manual cross-compilation:**
```bash
# Install targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build for each platform
cargo build --release --target x86_64-unknown-linux-gnu    # Linux
cargo build --release --target x86_64-pc-windows-gnu       # Windows
cargo build --release --target x86_64-apple-darwin         # macOS Intel
cargo build --release --target aarch64-apple-darwin        # macOS Apple Silicon
```

**Find binaries:**
```bash
target/x86_64-unknown-linux-gnu/release/lexi
target/x86_64-pc-windows-gnu/release/lexi.exe
target/x86_64-apple-darwin/release/lexi
target/aarch64-apple-darwin/release/lexi
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
3. Build all platforms: `./build.sh`
4. Create git tag: `git tag v1.x.x`
5. Push tag: `git push origin v1.x.x`
6. Create GitHub release with binaries from `release/` folder
7. Update documentation