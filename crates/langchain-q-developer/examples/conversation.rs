//! Example demonstrating multi-turn conversations with Q Developer
//!
//! Make sure you're logged in first: `q login`
//!
//! Run with: `cargo run --example conversation`

use langchain_q_developer::QDeveloperLLM;
use langchain_rust::language_models::llm::LLM;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("Amazon Q Developer - Multi-turn Conversation Example\n");

    // Create Q Developer LLM
    let llm = QDeveloperLLM::new().await?;

    // First message
    println!("Turn 1:");
    let prompt1 = "What is Rust programming language?";
    println!("User: {}\n", prompt1);
    let response1 = llm.invoke(prompt1).await?;
    println!("Q: {}\n", response1);

    // Get conversation ID from first message
    let conv_id = llm.get_conversation_id().await;
    if let Some(ref id) = conv_id {
        println!("Conversation ID: {}\n", id);
    }

    // Second message in same conversation
    println!("\nTurn 2:");
    let prompt2 = "Can you give me a simple example of ownership in Rust?";
    println!("User: {}\n", prompt2);
    let response2 = llm.invoke(prompt2).await?;
    println!("Q: {}\n", response2);

    // Verify we're using the same conversation
    let conv_id2 = llm.get_conversation_id().await;
    if conv_id == conv_id2 {
        println!("\n✓ Successfully maintained conversation context!");
    }

    Ok(())
}
