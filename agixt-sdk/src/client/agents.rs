//! Agent operations using /v1 endpoints with ID-based parameters.

use crate::error::Result;
use std::collections::HashMap;

impl super::AGiXTSDK {
    // ==================== Agents ====================

    /// Get list of all agents. Returns list of agents with their IDs.
    pub async fn get_agents(&self) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let response = self
            .client
            .get(&format!("{}/v1/agent", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct AgentsResponse {
            agents: Vec<HashMap<String, serde_json::Value>>,
        }

        let result: AgentsResponse = serde_json::from_str(&text)?;
        Ok(result.agents)
    }

    /// Get agent ID by name. Returns None if not found.
    pub async fn get_agent_id_by_name(&self, agent_name: &str) -> Result<Option<String>> {
        let agents = self.get_agents().await?;
        for agent in agents {
            if let Some(name) = agent.get("name").and_then(|v| v.as_str()) {
                if name == agent_name {
                    return Ok(agent.get("id").and_then(|v| v.as_str()).map(String::from));
                }
            }
        }
        Ok(None)
    }

    /// Add a new agent. Returns agent info including agent_id.
    pub async fn add_agent(
        &self,
        agent_name: &str,
        settings: Option<HashMap<String, serde_json::Value>>,
        commands: Option<HashMap<String, serde_json::Value>>,
        training_urls: Option<Vec<String>>,
    ) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/agent", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "agent_name": agent_name,
                "settings": settings.unwrap_or_default(),
                "commands": commands.unwrap_or_default(),
                "training_urls": training_urls.unwrap_or_default(),
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

    /// Import an agent configuration.
    pub async fn import_agent(
        &self,
        agent_name: &str,
        settings: Option<HashMap<String, serde_json::Value>>,
        commands: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/agent/import", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "agent_name": agent_name,
                "settings": settings.unwrap_or_default(),
                "commands": commands.unwrap_or_default(),
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

    /// Rename an agent by ID.
    pub async fn rename_agent(&self, agent_id: &str, new_name: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .patch(&format!("{}/v1/agent/{}", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({ "new_name": new_name }))
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Update agent settings by ID.
    pub async fn update_agent_settings(
        &self,
        agent_id: &str,
        settings: HashMap<String, serde_json::Value>,
        agent_name: Option<&str>,
    ) -> Result<String> {
        let response = self
            .client
            .put(&format!("{}/v1/agent/{}", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "agent_name": agent_name.unwrap_or(""),
                "settings": settings,
                "commands": {},
                "training_urls": [],
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

    /// Update agent commands by ID.
    pub async fn update_agent_commands(
        &self,
        agent_id: &str,
        commands: HashMap<String, serde_json::Value>,
    ) -> Result<String> {
        let response = self
            .client
            .put(&format!("{}/v1/agent/{}/commands", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({ "commands": commands }))
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

    /// Delete an agent by ID.
    pub async fn delete_agent(&self, agent_id: &str) -> Result<String> {
        let response = self
            .client
            .delete(&format!("{}/v1/agent/{}", self.base_uri, agent_id))
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

    /// Get agent configuration by ID.
    pub async fn get_agentconfig(&self, agent_id: &str) -> Result<HashMap<String, serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/agent/{}", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct AgentResponse {
            agent: HashMap<String, serde_json::Value>,
        }

        let result: AgentResponse = serde_json::from_str(&text)?;
        Ok(result.agent)
    }

    // ==================== Commands ====================

    /// Get available commands for an agent by ID.
    pub async fn get_commands(&self, agent_id: &str) -> Result<HashMap<String, serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/agent/{}/command", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct CommandsResponse {
            commands: HashMap<String, serde_json::Value>,
        }

        let result: CommandsResponse = serde_json::from_str(&text)?;
        Ok(result.commands)
    }

    /// Toggle a command for an agent by ID.
    pub async fn toggle_command(&self, agent_id: &str, command_name: &str, enable: bool) -> Result<String> {
        let response = self
            .client
            .patch(&format!("{}/v1/agent/{}/command", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "command_name": command_name,
                "enable": enable,
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

    /// Execute a command on an agent by ID.
    pub async fn execute_command(
        &self,
        agent_id: &str,
        command_name: &str,
        command_args: HashMap<String, serde_json::Value>,
        conversation_id: Option<&str>,
    ) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/agent/{}/command", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "command_name": command_name,
                "command_args": command_args,
                "conversation_name": conversation_id.unwrap_or(""),
            }))
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct ResponseWrapper {
            response: serde_json::Value,
        }

        let result: ResponseWrapper = serde_json::from_str(&text)?;
        Ok(result.response)
    }

    // ==================== Prompting ====================

    /// Send a prompt to an agent by ID.
    pub async fn prompt_agent(
        &self,
        agent_id: &str,
        prompt_name: &str,
        prompt_args: HashMap<String, serde_json::Value>,
    ) -> Result<String> {
        let response = self
            .client
            .post(&format!("{}/v1/agent/{}/prompt", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "prompt_name": prompt_name,
                "prompt_args": prompt_args,
            }))
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct ResponseWrapper {
            response: String,
        }

        let result: ResponseWrapper = serde_json::from_str(&text)?;
        Ok(result.response)
    }

    /// Send an instruction to an agent.
    pub async fn instruct(&self, agent_id: &str, user_input: &str, conversation_id: &str) -> Result<String> {
        let mut args = HashMap::new();
        args.insert("user_input".to_string(), serde_json::json!(user_input));
        args.insert("disable_memory".to_string(), serde_json::json!(true));
        args.insert("conversation_name".to_string(), serde_json::json!(conversation_id));

        self.prompt_agent(agent_id, "instruct", args).await
    }

    /// Chat with an agent.
    pub async fn chat(
        &self,
        agent_id: &str,
        user_input: &str,
        conversation_id: &str,
        context_results: Option<i32>,
    ) -> Result<String> {
        let mut args = HashMap::new();
        args.insert("user_input".to_string(), serde_json::json!(user_input));
        args.insert("context_results".to_string(), serde_json::json!(context_results.unwrap_or(4)));
        args.insert("conversation_name".to_string(), serde_json::json!(conversation_id));
        args.insert("disable_memory".to_string(), serde_json::json!(true));

        self.prompt_agent(agent_id, "Chat", args).await
    }

    // ==================== Persona ====================

    /// Get agent persona by ID.
    pub async fn get_persona(&self, agent_id: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(&format!("{}/v1/agent/{}/persona", self.base_uri, agent_id))
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
            message: serde_json::Value,
        }

        let result: MessageResponse = serde_json::from_str(&text)?;
        Ok(result.message)
    }

    /// Update agent persona by ID.
    pub async fn update_persona(&self, agent_id: &str, persona: &str) -> Result<String> {
        let response = self
            .client
            .put(&format!("{}/v1/agent/{}/persona", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({ "persona": persona }))
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

    // ==================== Extensions ====================

    /// Get extensions for an agent by ID.
    pub async fn get_agent_extensions(&self, agent_id: &str) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/agent/{}/extensions", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct ExtensionsResponse {
            extensions: Vec<serde_json::Value>,
        }

        let result: ExtensionsResponse = serde_json::from_str(&text)?;
        Ok(result.extensions)
    }

    // ==================== Feedback ====================

    /// Submit feedback for an agent response.
    pub async fn submit_feedback(
        &self,
        agent_id: &str,
        message: &str,
        user_input: &str,
        feedback: &str,
        positive: bool,
        conversation_id: Option<&str>,
    ) -> Result<String> {
        let response = self
            .client
            .post(&format!("{}/v1/agent/{}/feedback", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "user_input": user_input,
                "message": message,
                "feedback": feedback,
                "positive": positive,
                "conversation_name": conversation_id.unwrap_or(""),
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

    /// Submit positive feedback for an agent response.
    pub async fn positive_feedback(
        &self,
        agent_id: &str,
        message: &str,
        user_input: &str,
        feedback: &str,
        conversation_id: Option<&str>,
    ) -> Result<String> {
        self.submit_feedback(agent_id, message, user_input, feedback, true, conversation_id).await
    }

    /// Submit negative feedback for an agent response.
    pub async fn negative_feedback(
        &self,
        agent_id: &str,
        message: &str,
        user_input: &str,
        feedback: &str,
        conversation_id: Option<&str>,
    ) -> Result<String> {
        self.submit_feedback(agent_id, message, user_input, feedback, false, conversation_id).await
    }

    // ==================== Learning ====================

    /// Teach agent text content by ID.
    pub async fn learn_text(
        &self,
        agent_id: &str,
        user_input: &str,
        text: &str,
        collection_number: Option<&str>,
    ) -> Result<String> {
        let response = self
            .client
            .post(&format!("{}/v1/agent/{}/learn/text", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "user_input": user_input,
                "text": text,
                "collection_number": collection_number.unwrap_or("0"),
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

    /// Teach agent content from a URL by ID.
    pub async fn learn_url(
        &self,
        agent_id: &str,
        url: &str,
        collection_number: Option<&str>,
    ) -> Result<String> {
        let response = self
            .client
            .post(&format!("{}/v1/agent/{}/learn/url", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "url": url,
                "collection_number": collection_number.unwrap_or("0"),
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

    /// Teach agent content from a file by ID.
    pub async fn learn_file(
        &self,
        agent_id: &str,
        file_name: &str,
        file_content: &str,
        collection_number: Option<&str>,
    ) -> Result<String> {
        let response = self
            .client
            .post(&format!("{}/v1/agent/{}/learn/file", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "file_name": file_name,
                "file_content": file_content,
                "collection_number": collection_number.unwrap_or("0"),
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

    // ==================== Memory ====================

    /// Get agent memories.
    pub async fn get_agent_memories(
        &self,
        agent_id: &str,
        user_input: &str,
        limit: Option<i32>,
        min_relevance: Option<f32>,
        collection_number: Option<&str>,
    ) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .post(&format!("{}/v1/agent/{}/memory/query", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "user_input": user_input,
                "limit": limit.unwrap_or(10),
                "min_relevance_score": min_relevance.unwrap_or(0.0),
                "collection_number": collection_number.unwrap_or("0"),
            }))
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct MemoriesResponse {
            memories: Vec<serde_json::Value>,
        }

        let result: MemoriesResponse = serde_json::from_str(&text)?;
        Ok(result.memories)
    }

    /// Delete agent memory.
    pub async fn delete_agent_memory(
        &self,
        agent_id: &str,
        memory_id: &str,
        collection_number: Option<&str>,
    ) -> Result<String> {
        let response = self
            .client
            .delete(&format!("{}/v1/agent/{}/memory/{}", self.base_uri, agent_id, memory_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "collection_number": collection_number.unwrap_or("0"),
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

    /// Wipe agent memory.
    pub async fn wipe_agent_memory(
        &self,
        agent_id: &str,
        collection_number: Option<&str>,
    ) -> Result<String> {
        let response = self
            .client
            .delete(&format!("{}/v1/agent/{}/memory", self.base_uri, agent_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "collection_number": collection_number.unwrap_or(""),
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
