//! Example showing how to use Q Developer with custom configuration
//!
//! Make sure you're logged in first: `q login`
//!
//! Run with: `cargo run --example with_config`

use langchain_q_developer::types::QDeveloperConfig;
use langchain_q_developer::QDeveloperLLM;
use langchain_rust::language_models::llm::LLM;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("Amazon Q Developer - Custom Configuration Example\n");

    // Create custom configuration
    let config = QDeveloperConfig {
        model_id: None, // Use default model
        max_tokens: Some(2048),
        temperature: Some(0.7),
        stream: true,
        ..Default::default()
    };

    println!("Configuration:");
    println!("  Model: {:?}", config.model_id.as_deref().unwrap_or("default"));
    println!("  Max Tokens: {:?}", config.max_tokens);
    println!("  Temperature: {:?}", config.temperature);
    println!("  Streaming: {}\n", config.stream);

    // Create Q Developer LLM with config
    let llm = QDeveloperLLM::with_config(config).await?;

    // Ask a question
    let prompt = "Explain the difference between Vec and slice in Rust";
    println!("Prompt: {}\n", prompt);

    let response = llm.invoke(prompt).await?;
    println!("Response:\n{}\n", response);

    Ok(())
}
