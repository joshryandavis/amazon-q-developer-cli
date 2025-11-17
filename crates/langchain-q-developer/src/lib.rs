//! # LangChain Provider for Amazon Q Developer
//!
//! This crate provides a LangChain-compatible interface for Amazon Q Developer,
//! allowing you to use Q Developer as a language model backend in LangChain applications.
//!
//! ## Features
//!
//! - Uses existing Q CLI authentication (Builder ID or IAM)
//! - Supports streaming responses
//! - Compatible with LangChain's chat model interface
//! - Automatic conversation management
//!
//! ## Example
//!
//! ```no_run
//! use langchain_q_developer::QDeveloperLLM;
//! use langchain_rust::language_models::llm::LLM;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create Q Developer LLM (uses existing q CLI login)
//!     let llm = QDeveloperLLM::new().await?;
//!
//!     // Generate completion
//!     let response = llm.invoke("Write a hello world in Rust").await?;
//!     println!("{}", response);
//!
//!     Ok(())
//! }
//! ```

pub mod provider;
pub mod types;
pub mod error;

pub use provider::QDeveloperLLM;
pub use error::{QDeveloperError, Result};
