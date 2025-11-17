//! LangChain provider implementation for Amazon Q Developer

use async_trait::async_trait;
use futures::StreamExt;
use langchain_rust::language_models::llm::LLM;
use langchain_rust::language_models::GenerateResult;
use langchain_rust::schemas::messages::Message;
use langchain_rust::schemas::StreamData;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use crate::error::{QDeveloperError, Result};
use crate::types::{ConversationContext, QDeveloperConfig, ResponseMetadata};

/// LangChain-compatible LLM provider for Amazon Q Developer
///
/// This provider uses the existing Q CLI authentication and configuration,
/// so users must be logged in via `q login` before using this provider.
///
/// # Example
///
/// ```no_run
/// use langchain_q_developer::QDeveloperLLM;
/// use langchain_rust::language_models::llm::LLM;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let llm = QDeveloperLLM::new().await?;
/// let response = llm.invoke("Explain quantum computing").await?;
/// println!("{}", response);
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct QDeveloperLLM {
    config: Arc<QDeveloperConfig>,
    context: Arc<RwLock<ConversationContext>>,
    metadata: Arc<RwLock<ResponseMetadata>>,
}

impl QDeveloperLLM {
    /// Create a new Q Developer LLM with default configuration
    ///
    /// This will use the existing Q CLI authentication. Make sure you're logged in
    /// by running `q login` before using this.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Not logged in to Q CLI
    /// - Database initialization fails
    /// - API client creation fails
    pub async fn new() -> Result<Self> {
        Self::with_config(QDeveloperConfig::default()).await
    }

    /// Create a new Q Developer LLM with custom configuration
    ///
    /// # Example
    ///
    /// ```no_run
    /// use langchain_q_developer::{QDeveloperLLM, types::QDeveloperConfig};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = QDeveloperConfig {
    ///     model_id: Some("anthropic.claude-3-sonnet-20240229-v1:0".to_string()),
    ///     stream: true,
    ///     ..Default::default()
    /// };
    ///
    /// let llm = QDeveloperLLM::with_config(config).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn with_config(config: QDeveloperConfig) -> Result<Self> {
        // Validate that we can access Q CLI infrastructure
        Self::validate_q_cli_access().await?;

        Ok(Self {
            config: Arc::new(config),
            context: Arc::new(RwLock::new(ConversationContext::new())),
            metadata: Arc::new(RwLock::new(ResponseMetadata::default())),
        })
    }

    /// Validate that Q CLI is accessible and user is authenticated
    async fn validate_q_cli_access() -> Result<()> {
        // This will be implemented to check database access and auth status
        // For now, we'll return Ok and let the actual API calls fail if needed
        info!("Validating Q CLI access...");

        // TODO: Add actual validation by trying to create Database and ApiClient
        // let database = Database::new().await.map_err(QDeveloperError::db)?;
        // Check auth status, etc.

        Ok(())
    }

    /// Set the model ID to use for requests
    pub fn with_model(mut self, model_id: impl Into<String>) -> Self {
        Arc::make_mut(&mut self.config).model_id = Some(model_id.into());
        self
    }

    /// Enable or disable streaming
    pub fn with_streaming(mut self, enabled: bool) -> Self {
        Arc::make_mut(&mut self.config).stream = enabled;
        self
    }

    /// Set the conversation ID for continuing a conversation
    pub async fn with_conversation_id(self, conversation_id: impl Into<String>) -> Self {
        let mut context = self.context.write().await;
        context.conversation_id = Some(conversation_id.into());
        drop(context);
        self
    }

    /// Get the current conversation ID
    pub async fn get_conversation_id(&self) -> Option<String> {
        self.context.read().await.conversation_id.clone()
    }

    /// Get the last response metadata
    pub async fn get_metadata(&self) -> ResponseMetadata {
        self.metadata.read().await.clone()
    }

    /// Clear conversation history
    pub async fn clear_history(&self) {
        let mut context = self.context.write().await;
        context.conversation_id = None;
        context.messages.clear();
    }

    /// Internal method to send a message to Q Developer
    async fn send_to_q_developer(&self, prompt: &str) -> Result<String> {
        use chat_cli::api_client::model::{ConversationState, UserInputMessage};
        use chat_cli::api_client::ApiClient;
        use chat_cli::database::Database;
        use chat_cli::os::{Env, Fs};

        info!("Sending message to Q Developer");
        debug!("Prompt: {}", prompt);

        // Initialize Q CLI infrastructure
        let env = Env::new();
        let fs = Fs::new();
        let mut database = Database::new()
            .await
            .map_err(|e| QDeveloperError::db(format!("Failed to initialize database: {}", e)))?;

        // Create API client
        let api_client = ApiClient::new(&env, &fs, &mut database, None)
            .await
            .map_err(|e| QDeveloperError::init(format!("Failed to create API client: {}", e)))?;

        // Build conversation state
        let context = self.context.read().await;
        let conversation_id = context.conversation_id.clone();
        drop(context);

        let user_input = UserInputMessage {
            content: prompt.to_string(),
            user_input_message_context: None,
            user_intent: None,
            images: None,
            model_id: self.config.model_id.clone(),
        };

        let conversation = ConversationState {
            conversation_id,
            user_input_message: user_input,
            history: None, // TODO: Convert langchain messages to Q history
        };

        // Send message
        let mut response = api_client
            .send_message(conversation)
            .await
            .map_err(|e| QDeveloperError::api(format!("Failed to send message: {}", e)))?;

        // Collect response
        let mut full_response = String::new();
        let mut new_conversation_id = None;
        let mut new_utterance_id = None;
        let request_id = response.request_id().map(|s| s.to_string());

        while let Some(event) = response
            .recv()
            .await
            .map_err(|e| QDeveloperError::stream(format!("Stream error: {}", e)))?
        {
            match event {
                chat_cli::api_client::model::ChatResponseStream::AssistantResponseEvent { content } => {
                    full_response.push_str(&content);
                }
                chat_cli::api_client::model::ChatResponseStream::CodeEvent { content } => {
                    full_response.push_str(&content);
                }
                chat_cli::api_client::model::ChatResponseStream::MessageMetadataEvent {
                    conversation_id,
                    utterance_id,
                } => {
                    new_conversation_id = conversation_id;
                    new_utterance_id = utterance_id;
                }
                chat_cli::api_client::model::ChatResponseStream::InvalidStateEvent { reason, message } => {
                    error!("Invalid state: {} - {}", reason, message);
                    return Err(QDeveloperError::api(format!(
                        "Invalid state: {} - {}",
                        reason, message
                    )));
                }
                _ => {
                    // Ignore other event types for now
                    debug!("Received other event type (ignored)");
                }
            }
        }

        // Update metadata
        let mut metadata = self.metadata.write().await;
        metadata.conversation_id = new_conversation_id.clone();
        metadata.utterance_id = new_utterance_id;
        metadata.request_id = request_id;
        metadata.model_id = self.config.model_id.clone();
        drop(metadata);

        // Update context with conversation ID
        if let Some(conv_id) = new_conversation_id {
            let mut context = self.context.write().await;
            context.conversation_id = Some(conv_id);
        }

        info!("Received response from Q Developer ({} chars)", full_response.len());

        Ok(full_response)
    }
}

#[async_trait]
impl LLM for QDeveloperLLM {
    /// Invoke the LLM with a prompt
    async fn invoke(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error + Send>> {
        self.send_to_q_developer(prompt)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)
    }

    /// Generate a response with detailed results
    async fn generate(&self, messages: &[Message]) -> Result<GenerateResult, Box<dyn std::error::Error + Send>> {
        // Convert messages to a single prompt
        // In a more sophisticated implementation, we would maintain the full conversation history
        let prompt = messages
            .iter()
            .map(|m| m.content.clone())
            .collect::<Vec<_>>()
            .join("\n\n");

        let response = self
            .send_to_q_developer(&prompt)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;

        let metadata = self.get_metadata().await;
        let mut generation_info = serde_json::Map::new();

        if let Some(conv_id) = metadata.conversation_id {
            generation_info.insert("conversation_id".to_string(), serde_json::json!(conv_id));
        }
        if let Some(req_id) = metadata.request_id {
            generation_info.insert("request_id".to_string(), serde_json::json!(req_id));
        }
        if let Some(model_id) = metadata.model_id {
            generation_info.insert("model_id".to_string(), serde_json::json!(model_id));
        }

        Ok(GenerateResult {
            generation: response,
            tokens: None,
            logprobs: None,
        })
    }

    /// Stream responses (currently not implemented - returns regular response)
    async fn stream(
        &self,
        _prompt: &str,
    ) -> Result<
        std::pin::Pin<Box<dyn futures::Stream<Item = Result<StreamData, Box<dyn std::error::Error + Send>>> + Send>>,
        Box<dyn std::error::Error + Send>,
    > {
        // For now, return an error indicating streaming is not yet implemented
        // In the future, this could be implemented using Q Developer's streaming API
        Err(Box::new(QDeveloperError::config(
            "Streaming not yet implemented for Q Developer provider",
        )))
    }
}

impl std::fmt::Debug for QDeveloperLLM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QDeveloperLLM")
            .field("config", &self.config)
            .finish()
    }
}
