//! Type conversions between LangChain and Q Developer types

use langchain_rust::schemas::messages::{Message, MessageType};
use serde::{Deserialize, Serialize};

/// Configuration for Q Developer LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QDeveloperConfig {
    /// Model ID to use (e.g., "anthropic.claude-3-sonnet-20240229-v1:0")
    /// If None, uses the default model from the API
    pub model_id: Option<String>,

    /// Maximum number of tokens to generate
    pub max_tokens: Option<u32>,

    /// Temperature for sampling (0.0 to 1.0)
    pub temperature: Option<f32>,

    /// Custom endpoint URL (for testing or custom deployments)
    pub endpoint_url: Option<String>,

    /// Enable streaming responses
    pub stream: bool,
}

impl Default for QDeveloperConfig {
    fn default() -> Self {
        Self {
            model_id: None,
            max_tokens: None,
            temperature: None,
            endpoint_url: None,
            stream: true,
        }
    }
}

/// Conversation context for Q Developer
#[derive(Debug, Clone)]
pub struct ConversationContext {
    pub conversation_id: Option<String>,
    pub messages: Vec<Message>,
}

impl ConversationContext {
    pub fn new() -> Self {
        Self {
            conversation_id: None,
            messages: Vec::new(),
        }
    }

    pub fn with_conversation_id(mut self, id: String) -> Self {
        self.conversation_id = Some(id);
        self
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Convert to Q Developer chat history
    /// Returns (current_message, history)
    pub fn to_q_history(&self) -> Option<(String, Vec<QChatMessage>)> {
        if self.messages.is_empty() {
            return None;
        }

        let mut messages = self.messages.clone();
        let current = messages.pop()?;
        let current_content = message_to_string(&current);

        let history: Vec<QChatMessage> = messages
            .into_iter()
            .map(|msg| match msg.message_type {
                MessageType::HumanMessage => QChatMessage::User {
                    content: message_to_string(&msg),
                },
                MessageType::AIMessage => QChatMessage::Assistant {
                    content: message_to_string(&msg),
                },
                MessageType::SystemMessage => QChatMessage::System {
                    content: message_to_string(&msg),
                },
                _ => QChatMessage::User {
                    content: message_to_string(&msg),
                },
            })
            .collect();

        Some((current_content, history))
    }
}

impl Default for ConversationContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal representation of chat messages for Q Developer
#[derive(Debug, Clone)]
pub enum QChatMessage {
    User { content: String },
    Assistant { content: String },
    System { content: String },
}

/// Extract text content from a LangChain message
fn message_to_string(message: &Message) -> String {
    message.content.clone()
}

/// Response metadata from Q Developer
#[derive(Debug, Clone, Default)]
pub struct ResponseMetadata {
    pub conversation_id: Option<String>,
    pub utterance_id: Option<String>,
    pub request_id: Option<String>,
    pub model_id: Option<String>,
}
