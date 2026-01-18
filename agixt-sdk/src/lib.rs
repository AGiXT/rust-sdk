//! # AGiXT Rust SDK
//!
//! This is the official Rust SDK for AGiXT, providing a type-safe way to interact with the AGiXT API.
//! All endpoints use the /v1 API with ID-based parameters.
//!
//! ## Features
//!
//! - Full API coverage for AGiXT v1 endpoints
//! - Async/await support with tokio
//! - Type-safe request and response handling
//! - Comprehensive error handling
//! - ID-based resource management (agents, conversations, chains, prompts)
//!
//! ## Example
//!
//! ```rust,no_run
//! use agixt_sdk::AGiXTSDK;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a new SDK instance
//!     let client = AGiXTSDK::new(
//!         Some("http://localhost:7437".to_string()),
//!         Some("your-api-key".to_string()),
//!         false,
//!     );
//!
//!     // Get list of available providers
//!     let providers = client.get_providers().await?;
//!     println!("Available providers: {:?}", providers);
//!
//!     // Create a new agent and get its ID
//!     let agent_result = client.add_agent("my_agent", None, None, None).await?;
//!     let agent_id = agent_result["id"].as_str().unwrap();
//!     println!("Created agent with ID: {}", agent_id);
//!
//!     // Create a new conversation with the agent
//!     let conv_result = client.new_conversation(agent_id, "test_conversation", None).await?;
//!     let conversation_id = conv_result["id"].as_str().unwrap();
//!     println!("Created conversation with ID: {}", conversation_id);
//!
//!     // Chat with the agent
//!     let response = client.chat(agent_id, "Hello!", conversation_id, None).await?;
//!     println!("Agent response: {}", response);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! ```rust,no_run
//! use agixt_sdk::AGiXTSDK;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = AGiXTSDK::new(None, None, false);
//!
//!     // Register a new user
//!     let otp_uri = client.register_user(
//!         "user@example.com",
//!         "John",
//!         "Doe"
//!     ).await?;
//!     println!("Registration successful. OTP URI: {}", otp_uri);
//!
//!     // Login with email and OTP
//!     if let Some(token) = client.login("user@example.com", "123456").await? {
//!         println!("Login successful! Token: {}", token);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## ID-Based Resource Management
//!
//! The SDK uses ID-based parameters for all resource operations. Helper methods are provided
//! to look up IDs by name:
//!
//! ```rust,no_run
//! use agixt_sdk::AGiXTSDK;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = AGiXTSDK::new(None, Some("your-token".to_string()), false);
//!
//!     // Look up agent ID by name
//!     if let Some(agent_id) = client.get_agent_id_by_name("XT").await? {
//!         println!("Found agent with ID: {}", agent_id);
//!         
//!         // Use the ID for operations
//!         let config = client.get_agentconfig(&agent_id).await?;
//!         println!("Agent config: {:?}", config);
//!     }
//!
//!     // Look up conversation ID by name
//!     if let Some(conv_id) = client.get_conversation_id_by_name("My Chat").await? {
//!         let history = client.get_conversation(&conv_id, None, None).await?;
//!         println!("Conversation history: {:?}", history);
//!     }
//!
//!     // Look up chain ID by name
//!     if let Some(chain_id) = client.get_chain_id_by_name("Smart Instruct").await? {
//!         let chain_data = client.get_chain(&chain_id).await?;
//!         println!("Chain: {:?}", chain_data);
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod models;

pub use client::AGiXTSDK;
pub use error::{Error, Result};
pub use models::{
    Agent, Chain, ChainStep, ChatCompletions, ChatResponse, Choice, Company, ContentPart,
    Conversation, Extension, ExtensionCommand, FileUrl, ImageUrl, Message, MessageContent,
    Prompt, Provider, Tool, ToolFunction, Usage, User,
};
