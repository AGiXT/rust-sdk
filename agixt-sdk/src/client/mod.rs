mod agents;
mod conversations;
mod providers;

use crate::error::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

// Re-export functionality from submodules
pub use self::{
    agents::{Agent, AgentRequest},
    conversations::{ConversationHistory, Message},
    providers::{ProviderResponse, ProviderSettings},
};

#[derive(Clone)]
pub struct AGiXTSDK {
    base_uri: String,
    client: Arc<reqwest::Client>,
    headers: Arc<Mutex<HeaderMap>>,
    verbose: bool,
}

impl AGiXTSDK {
    pub fn new(base_uri: Option<String>, api_key: Option<String>, verbose: bool) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if let Some(key) = api_key {
            let api_key = key
                .replace("Bearer ", "")
                .replace("bearer ", "");
            if let Ok(value) = HeaderValue::from_str(&api_key) {
                headers.insert(AUTHORIZATION, value);
            }
        }

        let base_uri = base_uri.unwrap_or_else(|| "http://localhost:7437".to_string());
        let base_uri = base_uri.trim_end_matches('/').to_string();

        Self {
            base_uri,
            client: Arc::new(reqwest::Client::new()),
            headers: Arc::new(Mutex::new(headers)),
            verbose,
        }
    }

    /// Login to the AGiXT server
    pub async fn login(&self, email: &str, otp: &str) -> Result<Option<String>> {
        let response = self
            .client
            .post(&format!("{}/v1/login", self.base_uri))
            .json(&json!({
                "email": email,
                "token": otp,
            }))
            .send()
            .await?;

        if self.verbose {
            let status = response.status();
            let text = response.text().await?;
            self.parse_response(status, &text).await?;
            let json: serde_json::Value = serde_json::from_str(&text)?;
            self.process_login_response(json).await
        } else {
            let json = response.json::<serde_json::Value>().await?;
            self.process_login_response(json).await
        }
    }

    async fn process_login_response(&self, json: serde_json::Value) -> Result<Option<String>> {
        if let Some(detail) = json.get("detail").and_then(|d| d.as_str()) {
            if detail.contains("?token=") {
                let token = detail.split("token=").nth(1).unwrap_or_default();
                let mut headers = self.headers.lock().await;
                if let Ok(value) = HeaderValue::from_str(token) {
                    headers.insert(AUTHORIZATION, value);
                }
                println!("Log in at {}", detail);
                return Ok(Some(token.to_string()));
            }
        }
        Ok(None)
    }

    /// Register a new user
    pub async fn register_user(&self, email: &str, first_name: &str, last_name: &str) -> Result<String> {
        let response = self
            .client
            .post(&format!("{}/v1/user", self.base_uri))
            .json(&json!({
                "email": email,
                "first_name": first_name,
                "last_name": last_name,
            }))
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let json: serde_json::Value = serde_json::from_str(&text)?;
        
        if let Some(otp_uri) = json.get("otp_uri").and_then(|u| u.as_str()) {
            let mfa_token = otp_uri
                .split("secret=")
                .nth(1)
                .and_then(|s| s.split('&').next())
                .ok_or_else(|| crate::Error::Other("Invalid OTP URI format".to_string()))?;
            
            self.login(email, mfa_token).await?;
            
            Ok(otp_uri.to_string())
        } else {
            Ok(text)
        }
    }

    /// Check if a user exists
    pub async fn user_exists(&self, email: &str) -> Result<bool> {
        let response = self
            .client
            .get(&format!("{}/v1/user/exists", self.base_uri))
            .query(&[("email", email)])
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let json: serde_json::Value = serde_json::from_str(&text)?;
        Ok(json.as_bool().unwrap_or(false))
    }

    /// Update user information
    pub async fn update_user(&self, updates: serde_json::Value) -> Result<serde_json::Value> {
        let response = self
            .client
            .put(&format!("{}/v1/user", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&updates)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let json: serde_json::Value = serde_json::from_str(&text)?;
        Ok(json)
    }

    /// Get user information
    pub async fn get_user(&self) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(&format!("{}/v1/user", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let json: serde_json::Value = serde_json::from_str(&text)?;
        Ok(json)
    }

    /// Parse and log response if verbose mode is enabled
    pub(crate) async fn parse_response(
        &self,
        status: reqwest::StatusCode,
        body: &str,
    ) -> Result<()> {
        println!("Status Code: {}", status);
        println!("Response JSON:");
        
        if status.is_success() {
            println!("{}", body);
        } else {
            println!("{}", body);
            return Err(crate::Error::ApiError {
                status: status.as_u16(),
                message: body.to_string(),
            });
        }
        println!("\n");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_client() {
        let client = AGiXTSDK::new(None, None, false);
        assert_eq!(client.base_uri, "http://localhost:7437");
        assert!(!client.verbose);
    }

    #[tokio::test]
    async fn test_new_client_with_options() {
        let client = AGiXTSDK::new(
            Some("https://api.example.com/".to_string()),
            Some("test-key".to_string()),
            true,
        );
        assert_eq!(client.base_uri, "https://api.example.com");
        assert!(client.verbose);
    }
}