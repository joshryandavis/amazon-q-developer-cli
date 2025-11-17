//! Integration tests for langchain-q-developer
//!
//! Note: These tests require authentication with `q login`

use langchain_q_developer::{types::QDeveloperConfig, QDeveloperLLM};
use langchain_rust::language_models::llm::LLM;

#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored
async fn test_basic_invoke() {
    let llm = QDeveloperLLM::new().await.expect("Failed to create LLM");

    let response = llm
        .invoke("Say 'test successful' and nothing else")
        .await
        .expect("Failed to invoke");

    assert!(!response.is_empty());
    println!("Response: {}", response);
}

#[tokio::test]
#[ignore]
async fn test_with_config() {
    let config = QDeveloperConfig {
        model_id: None,
        stream: true,
        ..Default::default()
    };

    let llm = QDeveloperLLM::with_config(config)
        .await
        .expect("Failed to create LLM");

    let response = llm
        .invoke("What is 2+2? Answer with just the number.")
        .await
        .expect("Failed to invoke");

    assert!(!response.is_empty());
    println!("Response: {}", response);
}

#[tokio::test]
#[ignore]
async fn test_conversation_context() {
    let llm = QDeveloperLLM::new().await.expect("Failed to create LLM");

    // First message
    let response1 = llm
        .invoke("My favorite color is blue. Remember this.")
        .await
        .expect("Failed to invoke");

    let conv_id1 = llm.get_conversation_id().await;
    assert!(conv_id1.is_some());

    println!("Response 1: {}", response1);
    println!("Conversation ID: {:?}", conv_id1);

    // Second message - should remember context
    let response2 = llm
        .invoke("What is my favorite color?")
        .await
        .expect("Failed to invoke");

    let conv_id2 = llm.get_conversation_id().await;

    println!("Response 2: {}", response2);
    println!("Conversation ID: {:?}", conv_id2);

    // Should be the same conversation
    assert_eq!(conv_id1, conv_id2);
    assert!(response2.to_lowercase().contains("blue"));
}

#[tokio::test]
#[ignore]
async fn test_metadata() {
    let llm = QDeveloperLLM::new().await.expect("Failed to create LLM");

    let _response = llm.invoke("Hello").await.expect("Failed to invoke");

    let metadata = llm.get_metadata().await;

    assert!(metadata.conversation_id.is_some());
    println!("Metadata: {:?}", metadata);
}

#[tokio::test]
#[ignore]
async fn test_clear_history() {
    let llm = QDeveloperLLM::new().await.expect("Failed to create LLM");

    // Create a conversation
    llm.invoke("Remember: my name is Alice")
        .await
        .expect("Failed to invoke");

    let conv_id1 = llm.get_conversation_id().await;
    assert!(conv_id1.is_some());

    // Clear history
    llm.clear_history().await;

    let conv_id2 = llm.get_conversation_id().await;
    assert!(conv_id2.is_none());

    // New conversation should start
    llm.invoke("What is my name?").await.expect("Failed to invoke");

    let conv_id3 = llm.get_conversation_id().await;
    assert!(conv_id3.is_some());
    assert_ne!(conv_id1, conv_id3);
}
