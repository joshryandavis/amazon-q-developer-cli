# LangChain Provider for Amazon Q Developer

A [LangChain](https://github.com/Abraxas-365/langchain-rust) provider that integrates Amazon Q Developer as a language model backend. This allows you to use Q Developer's AI capabilities in your LangChain applications with automatic authentication using your existing Q CLI login.

## Features

- **Automatic Authentication**: Uses your existing `q` CLI login - no additional auth setup required
- **LangChain Compatible**: Implements the standard LangChain `LLM` trait
- **Conversation Management**: Maintains conversation context across multiple turns
- **Flexible Configuration**: Support for custom models, parameters, and endpoints
- **Metadata Access**: Access to conversation IDs, request IDs, and other metadata

## Prerequisites

You must be logged in to Amazon Q Developer via the CLI:

```bash
q login
```

This uses either:
- **Builder ID** (free tier) - OAuth authentication
- **AWS IAM Identity Center** - Enterprise SSO authentication

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
langchain-q-developer = "1.19.7"
langchain-rust = "4.6.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use langchain_q_developer::QDeveloperLLM;
use langchain_rust::language_models::llm::LLM;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Q Developer LLM (uses existing q CLI login)
    let llm = QDeveloperLLM::new().await?;

    // Generate completion
    let response = llm.invoke("Write a hello world in Rust").await?;
    println!("{}", response);

    Ok(())
}
```

## Usage Examples

### Simple Question-Answer

```rust
use langchain_q_developer::QDeveloperLLM;
use langchain_rust::language_models::llm::LLM;

let llm = QDeveloperLLM::new().await?;
let response = llm.invoke("Explain quantum computing").await?;
println!("{}", response);
```

### Multi-turn Conversations

```rust
use langchain_q_developer::QDeveloperLLM;
use langchain_rust::language_models::llm::LLM;

let llm = QDeveloperLLM::new().await?;

// First message
let response1 = llm.invoke("What is Rust?").await?;
println!("{}", response1);

// Conversation continues automatically
let response2 = llm.invoke("Can you show me an example?").await?;
println!("{}", response2);

// Get conversation ID
if let Some(conv_id) = llm.get_conversation_id().await {
    println!("Conversation ID: {}", conv_id);
}
```

### Custom Configuration

```rust
use langchain_q_developer::{QDeveloperLLM, types::QDeveloperConfig};

let config = QDeveloperConfig {
    model_id: Some("anthropic.claude-3-sonnet-20240229-v1:0".to_string()),
    max_tokens: Some(2048),
    temperature: Some(0.7),
    stream: true,
    ..Default::default()
};

let llm = QDeveloperLLM::with_config(config).await?;
```

### Accessing Metadata

```rust
let llm = QDeveloperLLM::new().await?;
let response = llm.invoke("Hello").await?;

let metadata = llm.get_metadata().await;
println!("Conversation ID: {:?}", metadata.conversation_id);
println!("Request ID: {:?}", metadata.request_id);
println!("Model ID: {:?}", metadata.model_id);
```

### Managing Conversations

```rust
let llm = QDeveloperLLM::new().await?;

// Start a conversation
llm.invoke("Tell me about Rust").await?;
let conv_id = llm.get_conversation_id().await;

// Clear history to start fresh
llm.clear_history().await;

// Or continue a specific conversation
let llm = QDeveloperLLM::new()
    .await?
    .with_conversation_id("previous-conv-id")
    .await;
```

## Configuration Options

The `QDeveloperConfig` struct supports the following options:

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `model_id` | `Option<String>` | `None` | Specific model to use (uses API default if None) |
| `max_tokens` | `Option<u32>` | `None` | Maximum tokens to generate |
| `temperature` | `Option<f32>` | `None` | Sampling temperature (0.0 - 1.0) |
| `endpoint_url` | `Option<String>` | `None` | Custom endpoint URL |
| `stream` | `bool` | `true` | Enable streaming responses |

## Examples

The crate includes several examples:

```bash
# Simple usage
cargo run --example simple

# Multi-turn conversation
cargo run --example conversation

# Custom configuration
cargo run --example with_config
```

## Architecture

This provider integrates with the Amazon Q CLI infrastructure:

```
┌─────────────────────────────────────┐
│   Your LangChain Application        │
└────────────┬────────────────────────┘
             │
             │ (uses LLM trait)
             ▼
┌─────────────────────────────────────┐
│   QDeveloperLLM (this crate)        │
│   - LangChain interface              │
│   - Conversation management          │
└────────────┬────────────────────────┘
             │
             │ (uses ApiClient)
             ▼
┌─────────────────────────────────────┐
│   Q CLI Infrastructure              │
│   - ApiClient                        │
│   - Authentication (Bearer/SigV4)   │
│   - Database & Settings              │
└─────────────────────────────────────┘
```

## Authentication

The provider uses your existing Q CLI authentication, which supports:

1. **Builder ID** - Free tier with OAuth device code flow
2. **IAM Identity Center** - Enterprise SSO

No additional configuration needed - just run `q login` first!

## Error Handling

The provider returns detailed errors:

```rust
use langchain_q_developer::QDeveloperError;

match llm.invoke("test").await {
    Ok(response) => println!("{}", response),
    Err(e) => match e.downcast_ref::<QDeveloperError>() {
        Some(QDeveloperError::AuthError(msg)) => {
            eprintln!("Not logged in: {}", msg);
            eprintln!("Run: q login");
        }
        Some(QDeveloperError::ApiError(msg)) => {
            eprintln!("API error: {}", msg);
        }
        _ => eprintln!("Error: {}", e),
    }
}
```

## Limitations

- **Streaming**: The LangChain `stream()` method is not yet implemented (returns error)
- **Message History**: Currently converts LangChain messages to simple prompts; full multi-modal history support coming soon
- **Tool Use**: Q Developer's tool use capabilities are not yet exposed through this interface

## Contributing

Contributions are welcome! This is part of the [amazon-q-developer-cli](https://github.com/aws/amazon-q-developer-cli) project.

## License

This project is dual-licensed under MIT and Apache 2.0. See [LICENSE.MIT](../../LICENSE.MIT) and [LICENSE.APACHE](../../LICENSE.APACHE).
