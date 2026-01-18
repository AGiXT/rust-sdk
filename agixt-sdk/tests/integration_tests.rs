//! AGiXT Rust SDK Integration Tests
//!
//! These tests run against a live AGiXT server.
//! Set the following environment variables:
//! - AGIXT_URI: AGiXT server URI (default: http://localhost:7437)
//! - AGIXT_API_KEY: API key for authentication (default: test-api-key)

use agixt_sdk::AGiXTSDK;
use std::env;
use uuid::Uuid;

fn get_sdk() -> AGiXTSDK {
    let base_uri = env::var("AGIXT_URI").unwrap_or_else(|_| "http://localhost:7437".to_string());
    let api_key = env::var("AGIXT_API_KEY").unwrap_or_else(|_| "test-api-key".to_string());
    AGiXTSDK::new(&base_uri, Some(&api_key)).expect("Failed to create SDK")
}

fn generate_unique_name(prefix: &str) -> String {
    format!("{}_{}", prefix, Uuid::new_v4().to_string()[..8].to_string())
}

mod connection_tests {
    use super::*;

    #[tokio::test]
    async fn test_server_reachable() {
        let sdk = get_sdk();
        let providers = sdk.get_providers().await;
        assert!(providers.is_ok(), "Server should be reachable");
    }
}

mod user_tests {
    use super::*;

    #[tokio::test]
    async fn test_register_user() {
        let sdk = get_sdk();
        let email = format!("test_{}@example.com", Uuid::new_v4().to_string()[..8].to_string());
        let result = sdk.register_user(&email, "Test", "User").await;
        assert!(result.is_ok(), "User registration should succeed");
    }

    #[tokio::test]
    async fn test_user_exists() {
        let sdk = get_sdk();
        let email = format!("existing_{}@example.com", Uuid::new_v4().to_string()[..8].to_string());
        let _ = sdk.register_user(&email, "Existing", "User").await;
        let exists = sdk.user_exists(&email).await;
        assert!(exists.is_ok(), "User exists check should succeed");
    }
}

mod agent_tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get_agents() {
        let sdk = get_sdk();
        let agents = sdk.get_agents().await;
        assert!(agents.is_ok(), "Get agents should succeed");
    }

    #[tokio::test]
    async fn test_add_agent() {
        let sdk = get_sdk();
        let agent_name = generate_unique_name("TestAgent");
        let mut settings = HashMap::new();
        settings.insert("provider".to_string(), serde_json::json!("default"));
        
        let result = sdk.add_agent(&agent_name, Some(settings), None, None).await;
        assert!(result.is_ok(), "Add agent should succeed");
        
        // Cleanup
        if let Ok(agent_id) = sdk.get_agent_id_by_name(&agent_name).await {
            let _ = sdk.delete_agent(&agent_id).await;
        }
    }

    #[tokio::test]
    async fn test_get_agent_id_by_name() {
        let sdk = get_sdk();
        let agent_name = generate_unique_name("TestAgent");
        let mut settings = HashMap::new();
        settings.insert("provider".to_string(), serde_json::json!("default"));
        
        let _ = sdk.add_agent(&agent_name, Some(settings), None, None).await;
        let agent_id = sdk.get_agent_id_by_name(&agent_name).await;
        assert!(agent_id.is_ok(), "Get agent ID by name should succeed");
        
        // Cleanup
        if let Ok(id) = agent_id {
            let _ = sdk.delete_agent(&id).await;
        }
    }
}

mod conversation_tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get_conversations() {
        let sdk = get_sdk();
        let conversations = sdk.get_conversations().await;
        assert!(conversations.is_ok(), "Get conversations should succeed");
    }

    #[tokio::test]
    async fn test_new_conversation() {
        let sdk = get_sdk();
        
        // Create agent first
        let agent_name = generate_unique_name("TestAgent");
        let mut settings = HashMap::new();
        settings.insert("provider".to_string(), serde_json::json!("default"));
        let _ = sdk.add_agent(&agent_name, Some(settings), None, None).await;
        let agent_id = sdk.get_agent_id_by_name(&agent_name).await.expect("Should get agent ID");
        
        // Create conversation
        let conv_name = generate_unique_name("TestConv");
        let result = sdk.new_conversation(&agent_id, &conv_name).await;
        assert!(result.is_ok(), "New conversation should succeed");
        
        // Cleanup
        if let Ok(conv_id) = sdk.get_conversation_id_by_name(&conv_name).await {
            let _ = sdk.delete_conversation(&conv_id).await;
        }
        let _ = sdk.delete_agent(&agent_id).await;
    }
}

mod provider_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_providers() {
        let sdk = get_sdk();
        let providers = sdk.get_providers().await;
        assert!(providers.is_ok(), "Get providers should succeed");
    }

    #[tokio::test]
    async fn test_get_providers_by_service() {
        let sdk = get_sdk();
        let providers = sdk.get_providers_by_service("llm").await;
        assert!(providers.is_ok(), "Get providers by service should succeed");
    }
}

mod chain_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_chains() {
        let sdk = get_sdk();
        let chains = sdk.get_chains().await;
        assert!(chains.is_ok(), "Get chains should succeed");
    }
}

mod prompt_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_prompts() {
        let sdk = get_sdk();
        let prompts = sdk.get_prompts("Default").await;
        assert!(prompts.is_ok(), "Get prompts should succeed");
    }

    #[tokio::test]
    async fn test_get_all_prompts() {
        let sdk = get_sdk();
        let prompts = sdk.get_all_prompts().await;
        assert!(prompts.is_ok(), "Get all prompts should succeed");
    }
}

mod extension_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_extensions() {
        let sdk = get_sdk();
        let extensions = sdk.get_extensions().await;
        assert!(extensions.is_ok(), "Get extensions should succeed");
    }
}
