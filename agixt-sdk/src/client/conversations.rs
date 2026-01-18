//! Conversation operations using /v1 endpoints with ID-based parameters.

use crate::error::Result;
use crate::models::Message;
use std::collections::HashMap;

impl super::AGiXTSDK {
    // ==================== Conversations ====================

    /// Get all conversations. Returns list with conversation IDs.
    pub async fn get_conversations(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/conversations", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        // Handle both list (v1) and dict (legacy) responses
        let data: serde_json::Value = serde_json::from_str(&text)?;
        if let Some(arr) = data.as_array() {
            return Ok(arr.clone());
        }
        if let Some(obj) = data.as_object() {
            if let Some(convs) = obj.get("conversations").and_then(|v| v.as_array()) {
                return Ok(convs.clone());
            }
        }
        Ok(vec![])
    }

    /// Get all conversations with their IDs.
    pub async fn get_conversations_with_ids(&self) -> Result<Vec<HashMap<String, String>>> {
        let response = self
            .client
            .get(&format!("{}/v1/conversations", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        // Parse as list of objects with id and name
        let data: serde_json::Value = serde_json::from_str(&text)?;
        let mut result = Vec::new();
        
        let conversations = if let Some(arr) = data.as_array() {
            arr.clone()
        } else if let Some(obj) = data.as_object() {
            obj.get("conversations_with_ids")
                .or(obj.get("conversations"))
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default()
        } else {
            vec![]
        };

        for conv in conversations {
            if let Some(obj) = conv.as_object() {
                let mut map = HashMap::new();
                if let Some(id) = obj.get("id").and_then(|v| v.as_str()) {
                    map.insert("id".to_string(), id.to_string());
                }
                if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                    map.insert("name".to_string(), name.to_string());
                }
                result.push(map);
            }
        }
        
        Ok(result)
    }

    /// Get conversation ID by name. Returns None if not found.
    pub async fn get_conversation_id_by_name(&self, conversation_name: &str) -> Result<Option<String>> {
        let conversations = self.get_conversations_with_ids().await?;
        for conv in conversations {
            if let Some(name) = conv.get("name") {
                if name == conversation_name {
                    return Ok(conv.get("id").cloned());
                }
            }
        }
        Ok(None)
    }

    /// Get conversation history by ID.
    pub async fn get_conversation(
        &self,
        conversation_id: &str,
        limit: Option<i32>,
        page: Option<i32>,
    ) -> Result<Vec<Message>> {
        let response = self
            .client
            .get(&format!("{}/v1/conversation/{}", self.base_uri, conversation_id))
            .headers(self.headers.lock().await.clone())
            .query(&[
                ("limit", limit.unwrap_or(100).to_string()),
                ("page", page.unwrap_or(1).to_string()),
            ])
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct ConversationResponse {
            conversation_history: Vec<Message>,
        }

        let result: ConversationResponse = serde_json::from_str(&text)?;
        Ok(result.conversation_history)
    }

    /// Fork a conversation from a specific message.
    pub async fn fork_conversation(
        &self,
        conversation_id: &str,
        message_id: &str,
    ) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/conversation/fork/{}/{}", self.base_uri, conversation_id, message_id))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Create a new conversation. Returns conversation with ID.
    pub async fn new_conversation(
        &self,
        agent_id: &str,
        conversation_name: &str,
        conversation_content: Option<Vec<Message>>,
    ) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/conversation", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "conversation_name": conversation_name,
                "agent_id": agent_id,
                "conversation_content": conversation_content.unwrap_or_default(),
            }))
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Rename a conversation by ID.
    pub async fn rename_conversation(
        &self,
        conversation_id: &str,
        new_name: &str,
    ) -> Result<serde_json::Value> {
        let response = self
            .client
            .put(&format!("{}/v1/conversation/{}", self.base_uri, conversation_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "new_conversation_name": new_name,
            }))
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Delete a conversation by ID.
    pub async fn delete_conversation(&self, conversation_id: &str) -> Result<String> {
        let response = self
            .client
            .delete(&format!("{}/v1/conversation/{}", self.base_uri, conversation_id))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct MessageResponse {
            message: String,
        }

        let result: MessageResponse = serde_json::from_str(&text)?;
        Ok(result.message)
    }

    /// Delete a message from a conversation by IDs.
    pub async fn delete_conversation_message(
        &self,
        conversation_id: &str,
        message_id: &str,
    ) -> Result<String> {
        let response = self
            .client
            .delete(&format!("{}/v1/conversation/{}/message/{}", self.base_uri, conversation_id, message_id))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct MessageResponse {
            message: String,
        }

        let result: MessageResponse = serde_json::from_str(&text)?;
        Ok(result.message)
    }

    /// Update a message in a conversation by IDs.
    pub async fn update_conversation_message(
        &self,
        conversation_id: &str,
        message_id: &str,
        new_message: &str,
    ) -> Result<String> {
        let response = self
            .client
            .put(&format!("{}/v1/conversation/{}/message/{}", self.base_uri, conversation_id, message_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "new_message": new_message,
            }))
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct MessageResponse {
            message: String,
        }

        let result: MessageResponse = serde_json::from_str(&text)?;
        Ok(result.message)
    }

    /// Add a new message to a conversation.
    pub async fn new_conversation_message(
        &self,
        role: &str,
        message: &str,
        conversation_id: &str,
    ) -> Result<String> {
        let response = self
            .client
            .post(&format!("{}/v1/conversation/{}/message", self.base_uri, conversation_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "role": role,
                "message": message,
            }))
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct MessageResponse {
            message: String,
        }

        let result: MessageResponse = serde_json::from_str(&text)?;
        Ok(result.message)
    }
}
