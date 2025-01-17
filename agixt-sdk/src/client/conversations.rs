use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationHistory {
    pub conversation_history: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl super::AGiXTSDK {
    /// Get list of conversations
    pub async fn get_conversations(&self, _agent_name: Option<&str>) -> Result<Vec<String>> {
        let response = self
            .client
            .get(&format!("{}/api/conversations", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(Deserialize)]
        struct ConversationsResponse {
            conversations: Vec<String>,
        }

        let result: ConversationsResponse = serde_json::from_str(&text)?;
        Ok(result.conversations)
    }

    /// Get conversations with IDs
    pub async fn get_conversations_with_ids(&self) -> Result<Vec<HashMap<String, String>>> {
        let response = self
            .client
            .get(&format!("{}/api/conversations", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(Deserialize)]
        struct ConversationsResponse {
            conversations_with_ids: Vec<HashMap<String, String>>,
        }

        let result: ConversationsResponse = serde_json::from_str(&text)?;
        Ok(result.conversations_with_ids)
    }

    /// Get conversation history
    pub async fn get_conversation(
        &self,
        agent_name: &str,
        conversation_name: &str,
        limit: Option<i32>,
        page: Option<i32>,
    ) -> Result<Vec<Message>> {
        let request = serde_json::json!({
            "conversation_name": conversation_name,
            "agent_name": agent_name,
            "limit": limit.unwrap_or(100),
            "page": page.unwrap_or(1),
        });

        let response = self
            .client
            .get(&format!("{}/api/conversation", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let result: ConversationHistory = serde_json::from_str(&text)?;
        Ok(result.conversation_history)
    }

    /// Fork a conversation
    pub async fn fork_conversation(&self, conversation_name: &str, message_id: &str) -> Result<String> {
        let request = serde_json::json!({
            "conversation_name": conversation_name,
            "message_id": message_id,
        });

        let response = self
            .client
            .post(&format!("{}/api/conversation/fork", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(Deserialize)]
        struct MessageResponse {
            message: String,
        }

        let result: MessageResponse = serde_json::from_str(&text)?;
        Ok(result.message)
    }

    /// Create a new conversation
    pub async fn new_conversation(
        &self,
        agent_name: &str,
        conversation_name: &str,
        conversation_content: Option<Vec<Message>>,
    ) -> Result<Vec<Message>> {
        let request = serde_json::json!({
            "conversation_name": conversation_name,
            "agent_name": agent_name,
            "conversation_content": conversation_content.unwrap_or_default(),
        });

        let response = self
            .client
            .post(&format!("{}/api/conversation", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let result: ConversationHistory = serde_json::from_str(&text)?;
        Ok(result.conversation_history)
    }

    /// Rename a conversation
    pub async fn rename_conversation(
        &self,
        agent_name: &str,
        conversation_name: &str,
        new_name: &str,
    ) -> Result<String> {
        let request = serde_json::json!({
            "conversation_name": conversation_name,
            "new_conversation_name": new_name,
            "agent_name": agent_name,
        });

        let response = self
            .client
            .put(&format!("{}/api/conversation", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(Deserialize)]
        struct ConversationResponse {
            conversation_name: String,
        }

        let result: ConversationResponse = serde_json::from_str(&text)?;
        Ok(result.conversation_name)
    }

    /// Delete a conversation
    pub async fn delete_conversation(&self, agent_name: &str, conversation_name: &str) -> Result<String> {
        let request = serde_json::json!({
            "conversation_name": conversation_name,
            "agent_name": agent_name,
        });

        let response = self
            .client
            .delete(&format!("{}/api/conversation", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(Deserialize)]
        struct MessageResponse {
            message: String,
        }

        let result: MessageResponse = serde_json::from_str(&text)?;
        Ok(result.message)
    }

    /// Add a new message to a conversation
    pub async fn new_conversation_message(
        &self,
        role: &str,
        message: &str,
        conversation_name: &str,
    ) -> Result<String> {
        let request = serde_json::json!({
            "role": role,
            "message": message,
            "conversation_name": conversation_name,
        });

        let response = self
            .client
            .post(&format!("{}/api/conversation/message", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(Deserialize)]
        struct MessageResponse {
            message: String,
        }

        let result: MessageResponse = serde_json::from_str(&text)?;
        Ok(result.message)
    }
}

#[cfg(test)]
mod tests {
    use crate::AGiXTSDK;
    use mockito;

    #[tokio::test]
    async fn test_get_conversations() {
        let mut mock_server = mockito::Server::new();
        let _mock = mock_server
            .mock("GET", "/api/conversations")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"conversations": ["conv1", "conv2"]}"#)
            .create();

        let client = AGiXTSDK::new(Some(mock_server.url()), None, false);
        let conversations = client.get_conversations(None).await.unwrap();
        
        assert_eq!(conversations, vec!["conv1", "conv2"]);
    }
}