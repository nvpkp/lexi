# 📚 Lexi

**English-to-code compiler powered by AI**

Write code by describing what you want in plain English. Lexi compiles your natural language descriptions into working code in any programming language.

```bash
# fibonacci.lxi
Create a function that generates the first n numbers in the Fibonacci sequence

# Compile to any language
lexi compile fibonacci.lxi --target python
lexi compile fibonacci.lxi --target javascript  
lexi compile fibonacci.lxi --target rust
```

## ✨ Features

- 🗣️ **Natural Language Programming** - Write code in plain English
- 🤖 **AI-Powered** - Uses GPT-4, Claude, or local models
- 🚀 **Zero Dependencies** - Single binary, no runtime required
- 🌍 **Cross-Platform** - Windows, macOS, Linux
- 📦 **Multi-Language Output** - JavaScript, Python, Rust, Go, Java, C++
- ⚡ **Fast** - Native Rust performance

## 🚀 Quick Start

### Installation

Download the binary for your platform:

**Linux (available now):**
```bash
curl -L https://github.com/nvpkp/lexi/raw/main/release/v1.0.0/lexi-linux -o lexi
chmod +x lexi
```

**Windows/macOS:** Coming soon via [GitHub Releases](https://github.com/nvpkp/lexi/releases)

### Configuration

```bash
# Create profiles for different accounts
./lexi profile create work
./lexi profile create personal

# Set up work profile (Anthropic)
./lexi profile use work
./lexi config set provider anthropic
./lexi config set model claude-3-5-sonnet-20241022
./lexi config set api_key sk-ant-your-work-key

# Set up personal profile (OpenAI)
./lexi profile use personal
./lexi config set provider openai
./lexi config set model gpt-4
./lexi config set api_key sk-your-personal-key

# Switch between profiles
./lexi profile use work      # Use work account
./lexi profile use personal  # Use personal account
```

### Your First Program

```bash
# Create project
./lexi init my-app
cd my-app

# Edit src/main.lxi with your English descriptions
# Then compile and run
./lexi compile src/main.lxi --target javascript --run
```

## 📖 Documentation

- [📥 Installation](docs/installation.md) - Get Lexi running
- [📚 Usage Guide](docs/usage.md) - How to write and compile Lexi programs  
- [⚙️ Configuration](docs/configuration.md) - Set up AI providers
- [🎯 Examples](docs/examples.md) - Real-world Lexi programs
- [🛠️ Development](docs/development.md) - Contributing to Lexi

## 🎯 Example Programs

**Web Server:**
```
Build a REST API server that handles GET /users and POST /users
Add CORS support and JSON response formatting
```

**Data Processing:**
```
Create a function that reads CSV files and calculates statistics
Handle missing values and generate summary reports
```

**Browser Testing:**
```
Build a test suite that logs into a website and verifies the dashboard
Take screenshots on failures and generate HTML reports
```

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.