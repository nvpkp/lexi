#!/bin/bash

# Lexi Cross-Platform Build Script
# Builds binaries for Linux, Windows, and macOS

set -e  # Exit on any error

echo "🚀 Building Lexi for all platforms..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Version from Cargo.toml or default
VERSION=${1:-"1.0.0"}

# Create release directory
RELEASE_DIR="release/v$VERSION"
mkdir -p "$RELEASE_DIR"

echo -e "${BLUE}📦 Installing required targets...${NC}"

# Install all required targets
rustup target add x86_64-unknown-linux-gnu   2>/dev/null || true
rustup target add x86_64-pc-windows-gnu      2>/dev/null || true
rustup target add x86_64-apple-darwin        2>/dev/null || true
rustup target add aarch64-apple-darwin       2>/dev/null || true

echo -e "${BLUE}🔨 Building for all platforms...${NC}"

# Build function
build_target() {
    local target=$1
    local output_name=$2
    
    echo -e "${YELLOW}Building for $target...${NC}"
    
    if cargo build --release --target "$target"; then
        # Copy and rename binary
        if [[ "$target" == *"windows"* ]]; then
            cp "target/$target/release/lexi.exe" "$RELEASE_DIR/$output_name"
        else
            cp "target/$target/release/lexi" "$RELEASE_DIR/$output_name"
        fi
        
        # Make executable (for Unix targets)
        chmod +x "$RELEASE_DIR/$output_name" 2>/dev/null || true
        
        echo -e "${GREEN}✅ Successfully built $output_name${NC}"
    else
        echo -e "${RED}❌ Failed to build for $target${NC}"
        return 1
    fi
}

# Build for all platforms
echo ""
build_target "x86_64-unknown-linux-gnu" "lexi-linux"
echo ""
# Temporarily skip Windows build due to cross-compilation issues
# build_target "x86_64-pc-windows-gnu" "lexi-windows.exe"
echo -e "${YELLOW}⏭️  Skipping Windows build (cross-compilation issues)${NC}"
echo ""
# Temporarily skip macOS builds due to cross-compilation issues  
# build_target "x86_64-apple-darwin" "lexi-macos-intel"
# build_target "aarch64-apple-darwin" "lexi-macos-arm"
echo -e "${YELLOW}⏭️  Skipping macOS builds (cross-compilation issues)${NC}"
echo -e "${BLUE}💡 Use GitHub Actions for multi-platform builds${NC}"

echo ""
echo -e "${GREEN}🎉 All builds completed!${NC}"
echo ""
echo -e "${BLUE}📁 Binaries available in: $RELEASE_DIR${NC}"
ls -la "$RELEASE_DIR"

# Calculate file sizes
echo ""
echo -e "${BLUE}📊 Binary sizes:${NC}"
for file in "$RELEASE_DIR"/*; do
    if [ -f "$file" ]; then
        size=$(du -h "$file" | cut -f1)
        name=$(basename "$file")
        echo "  $name: $size"
    fi
done

echo ""
echo -e "${YELLOW}💡 To test a binary:${NC}"
echo "  chmod +x $RELEASE_DIR/lexi-linux"
echo "  $RELEASE_DIR/lexi-linux --version"

echo ""
echo -e "${YELLOW}📦 To create a GitHub release:${NC}"
echo "  1. git tag v$VERSION"
echo "  2. git push origin v$VERSION"
echo "  3. Upload files from $RELEASE_DIR to GitHub release"