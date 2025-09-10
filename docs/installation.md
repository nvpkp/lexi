# 📥 Installation

## Prerequisites

None! Lexi is a single binary with zero runtime dependencies.

## Download Binary

### Quick Install

**Linux/macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/nvpkp/lexi/main/install.sh | sh
```

**Windows (PowerShell):**
```powershell
iwr https://raw.githubusercontent.com/nvpkp/lexi/main/install.ps1 | iex
```

### Manual Download

1. Go to [Releases](https://github.com/nvpkp/lexi/releases)
2. Download for your platform:
   - `lexi-linux` (Linux x64)
   - `lexi-windows.exe` (Windows x64)
   - `lexi-macos-intel` (macOS Intel)
   - `lexi-macos-arm` (macOS Apple Silicon)

3. Make executable and add to PATH:

**Linux/macOS:**
```bash
chmod +x lexi-linux
sudo mv lexi-linux /usr/local/bin/lexi
```

**Windows:**
Move to a folder in your PATH or add folder to PATH.

## Verify Installation

```bash
lexi --version
# Output: Lexi v1.0.0
```

## Available Platforms

- **Linux** (x86_64) - `lexi-linux`
- **Windows** (x86_64) - `lexi-windows.exe`  
- **macOS Intel** (x86_64) - `lexi-macos-intel`
- **macOS Apple Silicon** (ARM64) - `lexi-macos-arm`

## Next Steps

1. [Configure your AI provider](configuration.md)
2. [Create your first project](usage.md)