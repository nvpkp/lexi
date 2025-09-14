# ⚙️ Configuration

Configuration is stored in `~/.lexi/profiles.json`. API keys are stored locally and only sent to your chosen AI provider.

## 👥 Profile System

Lexi supports multiple profiles for different accounts and use cases:

```bash
# List all profiles
lexi profile list

# Create new profile
lexi profile create work
lexi profile create personal  

# Switch between profiles
lexi profile use work
lexi profile use personal

# Show current active profile
lexi profile current

# Set config for specific profile
lexi profile set work api_key sk-ant-your-work-key
```

## 🤖 AI Providers

### OpenAI (Recommended)
```bash
# Create OpenAI profile
lexi profile create openai
lexi profile use openai
lexi config set provider openai
lexi config set model gpt-4
lexi config set api_key sk-your-openai-key
```

Get API key: [OpenAI API Keys](https://platform.openai.com/api-keys)

**Models:** `gpt-4` (best), `gpt-3.5-turbo` (faster)

### Anthropic Claude
```bash
# Create Anthropic profile
lexi profile create work
lexi profile use work
lexi config set provider anthropic
lexi config set model claude-3-5-sonnet-20241022
lexi config set api_key sk-ant-your-key
```

Get API key: [Anthropic Console](https://console.anthropic.com/)

**Models:** 
- `claude-3-5-sonnet-20241022` (latest, recommended)
- `claude-3-5-sonnet-20240620` (stable)
- `claude-3-5-haiku-20241022` (fastest)
- `claude-3-opus-20240229` (most capable)

### Local (Ollama)
```bash
# Install Ollama from ollama.ai
ollama pull codellama

# Create local profile
lexi profile create local
lexi profile use local
lexi config set provider local
lexi config set model codellama
```

No API costs, runs locally for privacy.

### Azure OpenAI
```bash
# Create Azure profile
lexi profile create azure
lexi profile use azure
lexi config set provider azure
lexi config set model your-deployment-name
lexi config set api_key your-azure-key
lexi config set base_url https://your-resource.openai.azure.com
```

## 🎯 Common Profile Setups

### Work + Personal Setup
```bash
# Work account (Anthropic)
lexi profile create work
lexi profile use work
lexi config set provider anthropic
lexi config set model claude-3-5-sonnet-20241022
lexi config set api_key sk-ant-work-key

# Personal account (OpenAI)  
lexi profile create personal
lexi profile use personal
lexi config set provider openai
lexi config set model gpt-4
lexi config set api_key sk-personal-openai-key

# Switch between them
lexi profile use work      # Use work account
lexi profile use personal  # Use personal account
```

### Multi-Provider Setup
```bash
# Different providers for different use cases
lexi profile create fast    # Anthropic Haiku for quick tasks
lexi profile create smart   # OpenAI GPT-4 for complex tasks  
lexi profile create free    # Local Ollama for experiments
```

## ⚙️ Advanced Settings

```bash
lexi config set temperature 0.1     # Consistency (0.0-1.0)
lexi config set max_tokens 2000     # Response length
```

## 📁 Configuration Storage

- **Configuration file:** `~/.lexi/profiles.json`
- **Windows:** `%USERPROFILE%\.lexi\profiles.json`

## 🔄 Profile Management

```bash
# View all profiles and their settings
lexi profile list

# Delete unused profiles
lexi profile delete old-profile

# Configure profile without switching
lexi profile set work model claude-3-opus-20240229
```