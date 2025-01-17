use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    pub settings: HashMap<String, serde_json::Value>,
    pub commands: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct AgentRequest {
    pub agent_name: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub settings: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub commands: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub training_urls: Vec<String>,
}

impl super::AGiXTSDK {
    /// Add a new agent
    pub async fn add_agent(
        &self,
        agent_name: &str,
        settings: Option<HashMap<String, serde_json::Value>>,
        commands: Option<HashMap<String, serde_json::Value>>,
        training_urls: Option<Vec<String>>,
    ) -> Result<serde_json::Value> {
        let request = AgentRequest {
            agent_name: agent_name.to_string(),
            settings: settings.unwrap_or_default(),
            commands: commands.unwrap_or_default(),
            training_urls: training_urls.unwrap_or_default(),
        };

        let response = self
            .client
            .post(&format!("{}/api/agent", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Import an existing agent
    pub async fn import_agent(
        &self,
        agent_name: &str,
        settings: Option<HashMap<String, serde_json::Value>>,
        commands: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<serde_json::Value> {
        let request = serde_json::json!({
            "agent_name": agent_name,
            "settings": settings.unwrap_or_default(),
            "commands": commands.unwrap_or_default(),
        });

        let response = self
            .client
            .post(&format!("{}/api/agent/import", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Rename an agent
    pub async fn rename_agent(&self, agent_name: &str, new_name: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .patch(&format!("{}/api/agent/{}", self.base_uri, agent_name))
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

    /// Update agent settings
    pub async fn update_agent_settings(
        &self,
        agent_name: &str,
        settings: HashMap<String, serde_json::Value>,
    ) -> Result<String> {
        let response = self
            .client
            .put(&format!("{}/api/agent/{}", self.base_uri, agent_name))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "settings": settings,
                "agent_name": agent_name,
            }))
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

    /// Update agent commands
    pub async fn update_agent_commands(
        &self,
        agent_name: &str,
        commands: HashMap<String, serde_json::Value>,
    ) -> Result<String> {
        let response = self
            .client
            .put(&format!("{}/api/agent/{}/commands", self.base_uri, agent_name))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "commands": commands,
                "agent_name": agent_name,
            }))
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

    /// Delete an agent
    pub async fn delete_agent(&self, agent_name: &str) -> Result<String> {
        let response = self
            .client
            .delete(&format!("{}/api/agent/{}", self.base_uri, agent_name))
            .headers(self.headers.lock().await.clone())
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

    /// Get list of all agents
    pub async fn get_agents(&self) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let response = self
            .client
            .get(&format!("{}/api/agent", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(Deserialize)]
        struct AgentsResponse {
            agents: Vec<HashMap<String, serde_json::Value>>,
        }

        let result: AgentsResponse = serde_json::from_str(&text)?;
        Ok(result.agents)
    }

    /// Get agent configuration
    pub async fn get_agent_config(&self, agent_name: &str) -> Result<HashMap<String, serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/api/agent/{}", self.base_uri, agent_name))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(Deserialize)]
        struct AgentResponse {
            agent: HashMap<String, serde_json::Value>,
        }

        let result: AgentResponse = serde_json::from_str(&text)?;
        Ok(result.agent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AGiXTSDK;
    use mockito::{mock, Mock};

    fn setup_mock_server() -> (AGiXTSDK, Mock) {
        let mock_server = mock("POST", "/api/agent")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "Agent created successfully"}"#)
            .create();

        let client = AGiXTSDK::new(
            Some(mockito::server_url()),
            None,
            false,
        );

        (client, mock_server)
    }

    #[tokio::test]
    async fn test_add_agent() {
        let (client, _mock) = setup_mock_server();
        
        let result = client.add_agent("test_agent", None, None, None).await.unwrap();
        assert_eq!(result["message"], "Agent created successfully");
    }
}