//! Simple example of using the Q Developer LangChain provider
//!
//! Make sure you're logged in first: `q login`
//!
//! Run with: `cargo run --example simple`

use langchain_q_developer::QDeveloperLLM;
use langchain_rust::language_models::llm::LLM;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("Amazon Q Developer - LangChain Example\n");

    // Create Q Developer LLM (uses existing q CLI login)
    println!("Initializing Q Developer...");
    let llm = QDeveloperLLM::new().await?;

    // Simple question
    let prompt = "Write a hello world program in Rust";
    println!("\nPrompt: {}\n", prompt);

    println!("Generating response...\n");
    let response = llm.invoke(prompt).await?;

    println!("Response:\n{}\n", response);

    // Get metadata
    let metadata = llm.get_metadata().await;
    if let Some(conv_id) = metadata.conversation_id {
        println!("Conversation ID: {}", conv_id);
    }
    if let Some(req_id) = metadata.request_id {
        println!("Request ID: {}", req_id);
    }

    Ok(())
}
