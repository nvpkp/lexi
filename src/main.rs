use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "lexi")]
#[command(about = "📚 Lexi - English-to-code compiler powered by AI")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Serialize, Deserialize, Clone)]
struct ProfileConfig {
    active_profile: String,
    profiles: HashMap<String, Config>,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        let mut profiles = HashMap::new();
        profiles.insert("default".to_string(), Config::default());
        
        ProfileConfig {
            active_profile: "default".to_string(),
            profiles,
        }
    }
}

#[derive(Subcommand)]
enum ProfileCommands {
    /// List all profiles
    List,
    /// Create or switch to profile  
    Use { name: String },
    /// Create new profile
    Create { name: String },
    /// Delete profile
    Delete { name: String },
    /// Show current active profile
    Current,
    /// Set config for specific profile
    Set { profile: String, key: String, value: String },
}

#[derive(Subcommand)]
enum Commands {
    /// Compile .lxi file to executable code
    Compile {
        /// Input .lxi file
        input: String,
        /// Target language
        #[arg(short, long, default_value = "javascript")]
        target: String,
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
        /// Compile and run immediately
        #[arg(short, long)]
        run: bool,
    },
    /// Create new Lexi project
    Init {
        /// Project name
        project_name: String,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        config_command: ConfigCommands,
    },
    Profile {
        #[command(subcommand)]
        profile_command: ProfileCommands,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Set configuration value
    Set { key: String, value: String },
    /// List current configuration
    List,
    /// Show configuration setup guide
    Init,
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    provider: String,
    model: String,
    api_key: String,
    base_url: String,
    temperature: f32,
    max_tokens: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            api_key: String::new(),
            base_url: String::new(),
            temperature: 0.1,
            max_tokens: 2000,
        }
    }
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: String,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

struct LexiCompiler {
    config_path: PathBuf,
}

impl LexiCompiler {
    fn new() -> Self {
        let mut config_path = dirs::home_dir().expect("Could not find home directory");
        config_path.push(".lexi");
        config_path.push("config.json");
        
        LexiCompiler { config_path }
    }

    fn load_config(&self) -> Config {
        if self.config_path.exists() {
            let content = fs::read_to_string(&self.config_path)
                .unwrap_or_else(|_| "{}".to_string());
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Config::default()
        }
    }

    fn save_config(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(config)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    fn load_profile_config(&self) -> ProfileConfig {
        if self.config_path.exists() {
            let content = fs::read_to_string(&self.config_path)
                .unwrap_or_else(|_| "{}".to_string());
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            ProfileConfig::default()
        }
    }

    fn save_profile_config(&self, profile_config: &ProfileConfig) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(profile_config)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    async fn compile(&self, input: &str, target: &str, output: Option<&str>, run: bool) -> Result<(), Box<dyn std::error::Error>> {
        // Validate input file
        if !input.ends_with(".lxi") && !input.ends_with(".lexi") {
            eprintln!("❌ Error: Input file must have .lxi or .lexi extension");
            process::exit(1);
        }

        if !std::path::Path::new(input).exists() {
            eprintln!("❌ Error: File '{}' not found", input);
            process::exit(1);
        }

        println!("📚 Lexi v1.0.0 - Compiling {}...", input);

        // Read source file
        let lexi_content = fs::read_to_string(input)?;
        if lexi_content.trim().is_empty() {
            eprintln!("❌ Error: Source file is empty");
            process::exit(1);
        }

        // Generate output filename - create owned String
        let default_output = self.get_default_output_file(input, target);
        let output_file = output.unwrap_or(&default_output);

        println!("🤖 Generating code with AI...");
        
        // Generate code using LLM
        let generated_code = self.generate_code_with_llm(&lexi_content, target).await?;

        // Write output file
        fs::write(output_file, generated_code)?;
        println!("✅ Successfully compiled to {}", output_file);

        // Run if requested
        if run {
            println!("🚀 Running {}...", output_file);
            self.run_output(output_file, target)?;
        }

        Ok(())
    }

    fn get_default_output_file(&self, input: &str, target: &str) -> String {
        let base_name = std::path::Path::new(input)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        
        let extension = match target {
            "javascript" => ".js",
            "python" => ".py", 
            "java" => ".java",
            "cpp" => ".cpp",
            "rust" => ".rs",
            "go" => ".go",
            "sql" => ".sql",
            "mongodb" => ".js",
            "redis" => ".txt",
            _ => ".js",
        };

        format!("{}{}", base_name, extension)
    }

    async fn generate_code_with_llm(&self, lexi_content: &str, target: &str) -> Result<String, Box<dyn std::error::Error>> {
        let config = self.load_config();

        if config.api_key.is_empty() && config.provider != "local" {
            eprintln!("❌ No API key configured. Run: lexi config set api_key <your-key>");
            process::exit(1);
        }

        let (system_prompt, user_prompt) = self.build_prompt(lexi_content, target);

        let response = match config.provider.as_str() {
            "openai" => self.call_openai(&system_prompt, &user_prompt, &config).await?,
            "anthropic" => self.call_anthropic(&system_prompt, &user_prompt, &config).await?,
            "local" => self.call_ollama(&system_prompt, &user_prompt, &config).await?,
            "azure" => self.call_azure(&system_prompt, &user_prompt, &config).await?,
            _ => return Err(format!("Unsupported provider: {}", config.provider).into()),
        };

        Ok(self.extract_code_from_response(&response, target))
    }

    fn build_prompt(&self, lexi_content: &str, target: &str) -> (String, String) {
        let system_prompt = match target {
            "sql" => format!(
                "You are Lexi, a database query generator that converts English descriptions into clean, efficient SQL.

Rules:
1. Generate only the SQL code, no explanations or markdown
2. Use proper SQL syntax with appropriate JOINs, WHERE clauses, and indexing considerations
3. Include comments for complex queries
4. Use standard SQL that works across major databases (PostgreSQL, MySQL, SQL Server)
5. Generate complete, working SQL statements
6. Consider performance and use appropriate LIMIT clauses when needed

Target: SQL"
            ),
            "mongodb" => format!(
                "You are Lexi, a MongoDB query generator that converts English descriptions into MongoDB queries.

Rules:
1. Generate only MongoDB JavaScript code, no explanations or markdown
2. Use proper MongoDB syntax with appropriate aggregation pipelines
3. Include comments for complex operations
4. Use modern MongoDB methods and operators
5. Generate complete, working MongoDB queries
6. Consider performance with appropriate indexing hints

Target: MongoDB JavaScript"
            ),
            "redis" => format!(
                "You are Lexi, a Redis command generator that converts English descriptions into Redis commands.

Rules:
1. Generate only Redis commands, no explanations or markdown
2. Use proper Redis syntax and data structures
3. Include comments for complex operations
4. Use appropriate Redis commands for the use case
5. Generate complete, working Redis command sequences
6. Consider memory usage and TTL when appropriate

Target: Redis"
            ),
            _ => format!(
                "You are Lexi, a code generator that converts English descriptions into clean, functional {} code.

Rules:
1. Generate only the code, no explanations or markdown
2. Include proper error handling and edge cases
3. Use modern best practices for {}
4. Add structural comments but no console.log statements
5. Make functions keyboard accessible if UI-related
6. Generate complete, working implementations

Target language: {}",
                target, target, target
            )
        };

        let user_prompt = match target {
            "sql" => format!(
                "Convert this description into SQL:

{}

Generate clean, efficient SQL with proper syntax.",
                lexi_content
            ),
            "mongodb" => format!(
                "Convert this description into MongoDB JavaScript:

{}

Generate clean MongoDB queries with proper syntax.",
                lexi_content
            ),
            "redis" => format!(
                "Convert this description into Redis commands:

{}

Generate clean Redis commands with proper syntax.",
                lexi_content
            ),
            _ => format!(
                "Convert this Lexi description into {} code:

{}

Generate clean, production-ready code with proper function names and structure.",
                target, lexi_content
            )
        };

        (system_prompt, user_prompt)
    }

    async fn call_openai(&self, system_prompt: &str, user_prompt: &str, config: &Config) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        
        let request = OpenAIRequest {
            model: config.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            temperature: config.temperature,
            max_tokens: config.max_tokens,
        };

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("OpenAI API error: {}", error_text).into());
        }

        let openai_response: OpenAIResponse = response.json().await?;
        Ok(openai_response.choices[0].message.content.clone())
    }

    async fn call_anthropic(&self, system_prompt: &str, user_prompt: &str, config: &Config) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        
        let request = AnthropicRequest {
            model: config.model.clone(),
            max_tokens: config.max_tokens,
            messages: vec![Message {
                role: "user".to_string(),
                content: format!("{}\n\n{}", system_prompt, user_prompt),
            }],
        };

        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &config.api_key)
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Anthropic API error: {}", error_text).into());
        }

        let anthropic_response: AnthropicResponse = response.json().await?;
        Ok(anthropic_response.content[0].text.clone())
    }

    async fn call_ollama(&self, system_prompt: &str, user_prompt: &str, config: &Config) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let base_url = if config.base_url.is_empty() {
            "http://localhost:11434"
        } else {
            &config.base_url
        };

        let request = OllamaRequest {
            model: config.model.clone(),
            prompt: format!("{}\n\n{}", system_prompt, user_prompt),
            stream: false,
        };

        let response = client
            .post(&format!("{}/api/generate", base_url))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Ollama API error: {}", error_text).into());
        }

        let ollama_response: OllamaResponse = response.json().await?;
        Ok(ollama_response.response)
    }

    async fn call_azure(&self, system_prompt: &str, user_prompt: &str, config: &Config) -> Result<String, Box<dyn std::error::Error>> {
        if config.base_url.is_empty() {
            return Err("Azure baseUrl not configured. Set: lexi config set base_url https://your-resource.openai.azure.com".into());
        }

        let client = reqwest::Client::new();
        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version=2023-12-01-preview",
            config.base_url, config.model
        );

        let request = OpenAIRequest {
            model: config.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            temperature: config.temperature,
            max_tokens: config.max_tokens,
        };

        let response = client
            .post(&url)
            .header("api-key", &config.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Azure API error: {}", error_text).into());
        }

        let azure_response: OpenAIResponse = response.json().await?;
        Ok(azure_response.choices[0].message.content.clone())
    }

    fn extract_code_from_response(&self, response: &str, _target: &str) -> String {
        // Remove markdown code blocks if present
        let mut code = response
            .replace("```javascript", "")
            .replace("```python", "")
            .replace("```java", "")
            .replace("```cpp", "")
            .replace("```rust", "")
            .replace("```go", "")
            .replace("```", "");

        // Remove leading/trailing whitespace
        code = code.trim().to_string();

        // Find first line that looks like code
        let lines: Vec<&str> = code.lines().collect();
        let mut start_index = 0;
        let mut end_index = lines.len();

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("function ")
                || trimmed.starts_with("def ")
                || trimmed.starts_with("class ")
                || trimmed.starts_with("import ")
                || trimmed.starts_with("const ")
                || trimmed.starts_with("let ")
                || trimmed.starts_with("var ")
                || trimmed.starts_with("#include")
            {
                start_index = i;
                break;
            }
        }

        // Find last line that looks like code
        for (i, line) in lines.iter().enumerate().rev() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("Note:") && !trimmed.starts_with("Example:") {
                end_index = i + 1;
                break;
            }
        }

        lines[start_index..end_index].join("\n")
    }

    fn init_project(&self, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let project_path = std::path::Path::new(project_name);
        
        if project_path.exists() {
            eprintln!("❌ Error: Directory '{}' already exists", project_name);
            process::exit(1);
        }

        // Create project structure
        fs::create_dir_all(project_path.join("src"))?;
        fs::create_dir_all(project_path.join("build"))?;

        // Create sample .lxi file
        let sample_lexi = r#"# Sample Lexi Program
# Write your logic in plain English below

Create a function that takes a list of numbers and returns only the even ones greater than 10

Create a function to check if a string is a palindrome, ignoring case and spaces

Build a simple web server that responds with "Hello, World!" on GET requests to the root path
"#;

        fs::write(project_path.join("src/main.lxi"), sample_lexi)?;

        // Create lexi.config.json
        let config = serde_json::json!({
            "name": project_name,
            "version": "1.0.0",
            "defaultTarget": "javascript",
            "sourceDir": "src",
            "buildDir": "build",
            "targets": {
                "javascript": { "extension": ".js" },
                "python": { "extension": ".py" },
                "java": { "extension": ".java" }
            }
        });

        fs::write(
            project_path.join("lexi.config.json"),
            serde_json::to_string_pretty(&config)?,
        )?;

        // Create README
        let readme = format!(
            r#"# {}

A Lexi project - code generated from English descriptions using AI.

## Getting Started

1. Configure your LLM provider:
   ```bash
   lexi config set provider openai
   lexi config set model gpt-4
   lexi config set api_key sk-your-key-here
   ```

2. Write your logic in `src/main.lxi`

3. Compile:
   ```bash
   lexi compile src/main.lxi --target javascript
   ```

4. Run:
   ```bash
   node build/main.js
   ```

## Commands

- `lexi compile <file.lxi>` - Compile to JavaScript (default)
- `lexi compile <file.lxi> --target python` - Compile to Python  
- `lexi compile <file.lxi> --run` - Compile and run immediately
- `lexi config list` - Show current configuration

## Project Structure

```
{}/
├── src/           # Your .lxi source files
├── build/         # Compiled output
├── lexi.config.json
└── README.md
```
"#,
            project_name, project_name
        );

        fs::write(project_path.join("README.md"), readme)?;

        println!("📚 Created new Lexi project: {}", project_name);
        println!("📁 Project structure:");
        println!("   {}/", project_name);
        println!("   ├── src/main.lxi");
        println!("   ├── build/");
        println!("   ├── lexi.config.json");
        println!("   └── README.md");
        println!();
        println!("Next steps:");
        println!("   cd {}", project_name);
        println!("   lexi config init  # Configure your AI provider");
        println!("   lexi compile src/main.lxi");

        Ok(())
    }

    fn set_config(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = self.load_config();

        match key {
            "provider" => config.provider = value.to_string(),
            "model" => config.model = value.to_string(),
            "api_key" => config.api_key = value.to_string(),
            "base_url" => config.base_url = value.to_string(),
            "temperature" => config.temperature = value.parse().unwrap_or(0.1),
            "max_tokens" => config.max_tokens = value.parse().unwrap_or(2000),
            _ => {
                eprintln!("❌ Unknown config key: {}", key);
                process::exit(1);
            }
        }

        self.save_config(&config)?;
        println!("✅ Set {} = {}", key, value);

        if key == "api_key" {
            println!("🔐 API key saved securely to ~/.lexi/config.json");
        }

        Ok(())
    }

    fn list_config(&self) {
        let profile_config = self.load_profile_config();
        let default_config = Config::default();
        let config = profile_config.profiles
            .get(&profile_config.active_profile)
            .unwrap_or(&default_config);
            
        println!("🔧 Lexi Configuration (Profile: {}):", profile_config.active_profile);
        println!("   provider: {}", config.provider);
        println!("   model: {}", config.model);
        println!(
            "   api_key: {}",
            if config.api_key.is_empty() {
                "(not set)".to_string()
            } else {
                format!("{}...", &config.api_key[..8.min(config.api_key.len())])
            }
        );
        println!("   base_url: {}", if config.base_url.is_empty() { "(not set)" } else { &config.base_url });
        println!("   temperature: {}", config.temperature);
        println!("   max_tokens: {}", config.max_tokens);
    }

    fn list_profiles(&self) {
        let profile_config = self.load_profile_config();
        println!("👥 Available Profiles:");
        
        for (name, config) in &profile_config.profiles {
            let active_marker = if name == &profile_config.active_profile { " (active)" } else { "" };
            println!("   • {}{} - {} ({})", name, active_marker, config.provider, config.model);
        }
    }

    fn switch_profile(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut profile_config = self.load_profile_config();
        
        if !profile_config.profiles.contains_key(name) {
            eprintln!("❌ Profile '{}' does not exist. Create it first with: lexi profile create {}", name, name);
            process::exit(1);
        }
        
        profile_config.active_profile = name.to_string();
        self.save_profile_config(&profile_config)?;
        
        println!("✅ Switched to profile '{}'", name);
        Ok(())
    }

    fn create_profile(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut profile_config = self.load_profile_config();
        
        if profile_config.profiles.contains_key(name) {
            println!("⚠️  Profile '{}' already exists", name);
            return Ok(());
        }
        
        profile_config.profiles.insert(name.to_string(), Config::default());
        profile_config.active_profile = name.to_string();
        self.save_profile_config(&profile_config)?;
        
        println!("✅ Created and switched to profile '{}'", name);
        println!("💡 Configure it with: lexi config set <key> <value>");
        Ok(())
    }

    fn delete_profile(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut profile_config = self.load_profile_config();
        
        if name == "default" {
            eprintln!("❌ Cannot delete the default profile");
            process::exit(1);
        }
        
        if !profile_config.profiles.contains_key(name) {
            eprintln!("❌ Profile '{}' does not exist", name);
            process::exit(1);
        }
        
        profile_config.profiles.remove(name);
        
        // Switch to default if we deleted the active profile
        if profile_config.active_profile == name {
            profile_config.active_profile = "default".to_string();
            println!("🔄 Switched back to 'default' profile");
        }
        
        self.save_profile_config(&profile_config)?;
        println!("✅ Deleted profile '{}'", name);
        Ok(())
    }

    fn show_current_profile(&self) {
        let profile_config = self.load_profile_config();
        println!("📍 Current profile: {}", profile_config.active_profile);
    }

    fn set_profile_config(&self, profile_name: &str, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut profile_config = self.load_profile_config();
        
        if !profile_config.profiles.contains_key(profile_name) {
            eprintln!("❌ Profile '{}' does not exist. Create it first with: lexi profile create {}", profile_name, profile_name);
            process::exit(1);
        }
        
        let mut config = profile_config.profiles[profile_name].clone();
        
        match key {
            "provider" => config.provider = value.to_string(),
            "model" => config.model = value.to_string(),
            "api_key" => config.api_key = value.to_string(),
            "base_url" => config.base_url = value.to_string(),
            "temperature" => config.temperature = value.parse().unwrap_or(0.1),
            "max_tokens" => config.max_tokens = value.parse().unwrap_or(2000),
            _ => {
                eprintln!("❌ Unknown config key: {}", key);
                process::exit(1);
            }
        }
        
        profile_config.profiles.insert(profile_name.to_string(), config);
        self.save_profile_config(&profile_config)?;
        
        println!("✅ Set {} = {} for profile '{}'", key, value, profile_name);
        
        if key == "api_key" {
            println!("🔐 API key saved securely");
        }
        
        Ok(())
    }

    fn show_config_init(&self) {
        println!("🚀 Setting up Lexi configuration...");
        println!();
        println!("Available providers:");
        println!("  • openai (GPT-4, GPT-3.5)");
        println!("  • anthropic (Claude)");
        println!("  • local (Ollama)");
        println!("  • azure (Azure OpenAI)");
        println!();
        println!("Example setup:");
        println!("  lexi config set provider openai");
        println!("  lexi config set model gpt-4");
        println!("  lexi config set api_key sk-...");
        println!();
        println!("For local Ollama:");
        println!("  lexi config set provider local");
        println!("  lexi config set model codellama");
        println!("  lexi config set base_url http://localhost:11434");
    }

    fn run_output(&self, output_file: &str, target: &str) -> Result<(), Box<dyn std::error::Error>> {
        match target {
            "javascript" => {
                process::Command::new("node")
                    .arg(output_file)
                    .status()?;
            }
            "python" => {
                process::Command::new("python")
                    .arg(output_file)
                    .status()?;
            }
            _ => {
                println!("⚠️  Auto-run not supported for {} yet", target);
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let compiler = LexiCompiler::new();

    match cli.command {
        Commands::Compile { input, target, output, run } => {
            compiler.compile(&input, &target, output.as_deref(), run).await?;
        }
        Commands::Init { project_name } => {
            compiler.init_project(&project_name)?;
        }
        Commands::Config { config_command } => match config_command {
            ConfigCommands::Set { key, value } => {
                compiler.set_config(&key, &value)?;
            }
            ConfigCommands::List => {
                compiler.list_config();
            }
            ConfigCommands::Init => {
                compiler.show_config_init();
            }
        },
        Commands::Profile { profile_command } => match profile_command {
            ProfileCommands::List => {
                compiler.list_profiles();
            }
            ProfileCommands::Use { name } => {
                compiler.switch_profile(&name)?;
            }
            ProfileCommands::Create { name } => {
                compiler.create_profile(&name)?;
            }
            ProfileCommands::Delete { name } => {
                compiler.delete_profile(&name)?;
            }
            ProfileCommands::Current => {
                compiler.show_current_profile();
            }
            ProfileCommands::Set { profile, key, value } => {
                compiler.set_profile_config(&profile, &key, &value)?;
            }
        },
    }

    Ok(())
}