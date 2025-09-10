# 📥 Installation

## Prerequisites

None! Lexi is a single binary with zero runtime dependencies.

## Download Binary

### Quick Install

**Linux/macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/yourusername/lexi/main/install.sh | sh
```

**Windows (PowerShell):**
```powershell
iwr https://raw.githubusercontent.com/yourusername/lexi/main/install.ps1 | iex
```

### Manual Download

1. Go to [Releases](https://github.com/yourusername/lexi/releases)
2. Download for your platform:
   - `lexi-linux` (Linux x64)
   - `lexi-macos` (macOS)  
   - `lexi-windows.exe` (Windows)

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

## Next Steps

1. [Configure your AI provider](configuration.md)
2. [Create your first project](usage.md)
