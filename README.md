# AGiXT Rust SDK

This is the official Rust SDK for [AGiXT](https://github.com/AGiXT/python-sdk), providing a type-safe way to interact with the AGiXT API.

## Features

- Full API coverage for AGiXT
- Async/await support using Tokio
- Type-safe request and response handling
- Comprehensive error handling
- Built-in support for authentication and session management

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
agixt-sdk = "0.1.0"
```

## Quick Start

```rust
use agixt_sdk::AGiXTSDK;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new SDK instance
    let client = AGiXTSDK::new(
        Some("http://localhost:7437".to_string()),
        Some("your-api-key".to_string()),
        false,
    );

    // Get list of available providers
    let providers = client.get_providers().await?;
    println!("Available providers: {:?}", providers);

    // Create a new agent
    let agent_name = "my_agent";
    client.add_agent(agent_name, None, None, None).await?;

    // Start a new conversation
    let conversation = client.new_conversation(agent_name, "test_conversation", None).await?;
    println!("Created conversation: {:?}", conversation);

    Ok(())
}
```

## Authentication

```rust
use agixt_sdk::AGiXTSDK;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AGiXTSDK::new(None, None, false);

    // Register a new user
    let otp_uri = client.register_user(
        "user@example.com",
        "John",
        "Doe"
    ).await?;
    println!("Registration successful. OTP URI: {}", otp_uri);

    // Login with email and OTP
    if let Some(token) = client.login("user@example.com", "123456").await? {
        println!("Login successful! Token: {}", token);
    }

    Ok(())
}
```

## Core Features

### Providers

```rust
// Get all available providers
let providers = client.get_providers().await?;

// Get providers for a specific service
let chat_providers = client.get_providers_by_service("chat").await?;

// Get provider settings
let settings = client.get_provider_settings("gpt4free").await?;
```

### Agents

```rust
// Create a new agent
client.add_agent("my_agent", None, None, None).await?;

// Get agent configuration
let config = client.get_agent_config("my_agent").await?;

// Update agent settings
use std::collections::HashMap;
let mut settings = HashMap::new();
settings.insert("setting_key".to_string(), serde_json::json!("value"));
client.update_agent_settings("my_agent", settings).await?;
```

### Conversations

```rust
// Create a new conversation
let conversation = client.new_conversation("my_agent", "test_conv", None).await?;

// Add a message to the conversation
client.new_conversation_message("user", "Hello!", "test_conv").await?;

// Get conversation history
let history = client.get_conversation("my_agent", "test_conv", Some(10), Some(1)).await?;
```

## Error Handling

The SDK uses a custom error type that covers various error cases:

```rust
pub enum Error {
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
    ApiError { status: u16, message: String },
    AuthError(String),
    InvalidInput(String),
    Other(String),
}
```

All methods return a `Result<T, Error>` type, allowing for proper error handling:

```rust
match client.get_providers().await {
    Ok(providers) => println!("Providers: {:?}", providers),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.