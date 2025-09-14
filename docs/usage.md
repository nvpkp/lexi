# 📚 Usage Guide

## Basic Workflow

1. Write English descriptions in `.lxi` files
2. Compile to your target language  
3. Run the generated code

## Commands

### Compile
```bash
lexi compile <file.lxi> [options]

# Options:
--target, -t <language>   # Target language (default: javascript)
--output, -o <file>       # Output file path
--run, -r                 # Compile and run immediately
```

### Initialize Project
```bash
lexi init <project-name>
```

Creates project with sample files and structure.

### Configuration
```bash
lexi config set <key> <value>    # Set config for current profile
lexi config list                 # Show current profile config
```

### Profile Management
```bash
lexi profile list                    # List all profiles
lexi profile use <name>              # Switch to profile
lexi profile create <name>           # Create new profile
lexi profile current                 # Show active profile
lexi profile set <profile> <key> <value>  # Set config for specific profile
lexi profile delete <name>           # Delete profile
```

## Writing Lexi Programs

### File Structure
```
# Comments start with #
# Blank lines separate different functions

Create a function that calculates the factorial of a number

Create a function that checks if a number is prime
```

### Best Practices

**Be specific:**
```
❌ Create a sorting function
✅ Create a function that sorts an array of objects by a property in descending order
```

**Include edge cases:**
```
✅ Create a function that validates emails and handles empty strings and invalid formats
```

**Specify behavior:**
```
✅ Create a function that fetches data from an API with retry logic and timeout handling
```

## Target Languages

- **JavaScript** - `lexi compile app.lxi --target javascript`
- **Python** - `lexi compile app.lxi --target python`
- **Rust** - `lexi compile app.lxi --target rust`
- **Go** - `lexi compile app.lxi --target go`
- **Java** - `lexi compile app.lxi --target java`
- **C++** - `lexi compile app.lxi --target cpp`

## Project Structure

```
my-project/
├── src/
│   ├── main.lxi          # Main program
│   └── utils.lxi         # Utility functions
├── build/                # Compiled output
└── lexi.config.json      # Project settings
```

## Tips

- Start simple and build complexity
- Be explicit about requirements
- Test frequently with `--run`
- Use comments to document intent