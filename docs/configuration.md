# ⚙️ Configuration

Configuration is stored in `~/.lexi/config.json`. API keys are stored locally and only sent to your chosen AI provider.

## AI Providers

### OpenAI (Recommended)
```bash
lexi config set provider openai
lexi config set model gpt-4
lexi config set api_key sk-your-openai-key
```

Get API key: [OpenAI API Keys](https://platform.openai.com/api-keys)

**Models:** `gpt-4` (best), `gpt-3.5-turbo` (faster)

### Anthropic Claude
```bash
lexi config set provider anthropic
lexi config set model claude-3-sonnet-20240229
lexi config set api_key sk-ant-your-key
```

Get API key: [Anthropic Console](https://console.anthropic.com/)

**Models:** `claude-3-opus-20240229`, `claude-3-sonnet-20240229`, `claude-3-haiku-20240307`

### Local (Ollama)
```bash
# Install Ollama from ollama.ai
ollama pull codellama

# Configure Lexi
lexi config set provider local
lexi config set model codellama
```

No API costs, runs locally for privacy.

### Azure OpenAI
```bash
lexi config set provider azure
lexi config set model your-deployment-name
lexi config set api_key your-azure-key
lexi config set base_url https://your-resource.openai.azure.com
```

## Advanced Settings

```bash
lexi config set temperature 0.1     # Consistency (0.0-1.0)
lexi config set max_tokens 2000     # Response length
```

## View Configuration

```bash
lexi config list
```

## Configuration Location

- **Linux/macOS:** `~/.lexi/config.json`
- **Windows:** `%USERPROFILE%\.lexi\config.json`
