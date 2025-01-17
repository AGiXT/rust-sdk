//! # AGiXT Rust SDK
//! 
//! This is the official Rust SDK for AGiXT, providing a type-safe way to interact with the AGiXT API.
//! 
//! ## Features
//! 
//! - Full API coverage for AGiXT
//! - Async/await support
//! - Type-safe request and response handling
//! - Comprehensive error handling
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
//!     // Create a new agent
//!     let agent_name = "my_agent";
//!     client.add_agent(agent_name, None, None, None).await?;
//! 
//!     // Start a new conversation
//!     let conversation = client.new_conversation(agent_name, "test_conversation", None).await?;
//!     println!("Created conversation: {:?}", conversation);
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

pub mod error;
pub mod models;
pub mod client;

pub use client::AGiXTSDK;
pub use error::Error;

// Re-export commonly used types
pub use client::{
    Agent,
    AgentRequest,
    ConversationHistory,
    Message,
    ProviderResponse,
    ProviderSettings,
};