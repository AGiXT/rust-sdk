//! AGiXT SDK client implementation using /v1 endpoints with ID-based parameters.

mod agents;
mod conversations;
mod providers;

use crate::error::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// AGiXT SDK client for interacting with the AGiXT API.
#[derive(Clone)]
pub struct AGiXTSDK {
    pub(crate) base_uri: String,
    pub(crate) client: Arc<reqwest::Client>,
    pub(crate) headers: Arc<Mutex<HeaderMap>>,
    pub(crate) verbose: bool,
}

impl AGiXTSDK {
    /// Create a new AGiXT SDK instance.
    ///
    /// # Arguments
    /// * `base_uri` - Optional base URI for the AGiXT server (defaults to http://localhost:7437)
    /// * `api_key` - Optional API key or JWT token for authentication
    /// * `verbose` - Whether to print verbose debug output
    pub fn new(base_uri: Option<String>, api_key: Option<String>, verbose: bool) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if let Some(key) = api_key {
            let api_key = key.replace("Bearer ", "").replace("bearer ", "");
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

    // ==================== Authentication ====================

    /// Login with username/password authentication.
    ///
    /// # Arguments
    /// * `username` - Username or email address
    /// * `password` - User's password
    /// * `mfa_token` - Optional TOTP code if MFA is enabled
    ///
    /// # Returns
    /// JWT token on success, or None on failure
    pub async fn login(&self, username: &str, password: &str, mfa_token: Option<&str>) -> Result<Option<String>> {
        let mut payload = serde_json::json!({
            "username": username,
            "password": password,
        });
        if let Some(token) = mfa_token {
            payload["mfa_token"] = serde_json::json!(token);
        }

        let response = self
            .client
            .post(&format!("{}/v1/login", self.base_uri))
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let json: serde_json::Value = serde_json::from_str(&text)?;
        
        // Check for token in response (new auth flow)
        if status.is_success() {
            if let Some(token) = json.get("token").and_then(|t| t.as_str()) {
                let mut headers = self.headers.lock().await;
                if let Ok(value) = HeaderValue::from_str(token) {
                    headers.insert(AUTHORIZATION, value);
                }
                if self.verbose {
                    println!("Logged in successfully");
                }
                return Ok(Some(token.to_string()));
            }
        }
        Ok(None)
    }

    /// Legacy login with magic link (email + OTP token).
    /// Maintained for backward compatibility.
    ///
    /// # Arguments
    /// * `email` - User's email address
    /// * `otp` - TOTP code from authenticator app
    ///
    /// # Returns
    /// JWT token on success, or None on failure
    pub async fn login_magic_link(&self, email: &str, otp: &str) -> Result<Option<String>> {
        let response = self
            .client
            .post(&format!("{}/v1/login/magic-link", self.base_uri))
            .json(&serde_json::json!({
                "email": email,
                "token": otp,
            }))
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let json: serde_json::Value = serde_json::from_str(&text)?;
        
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

    /// Register a new user with username/password authentication.
    ///
    /// # Arguments
    /// * `email` - User's email address
    /// * `password` - User's password
    /// * `confirm_password` - Password confirmation
    /// * `first_name` - User's first name (optional)
    /// * `last_name` - User's last name (optional)
    /// * `username` - Desired username (optional)
    /// * `organization_name` - Company/organization name (optional)
    ///
    /// # Returns
    /// Response JSON with user_id, username, token on success
    pub async fn register_user(
        &self,
        email: &str,
        password: &str,
        confirm_password: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        username: Option<&str>,
        organization_name: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut payload = serde_json::json!({
            "email": email,
            "password": password,
            "confirm_password": confirm_password,
            "first_name": first_name.unwrap_or(""),
            "last_name": last_name.unwrap_or(""),
        });
        if let Some(u) = username {
            payload["username"] = serde_json::json!(u);
        }
        if let Some(org) = organization_name {
            payload["organization_name"] = serde_json::json!(org);
        }

        let response = self
            .client
            .post(&format!("{}/v1/user", self.base_uri))
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let json: serde_json::Value = serde_json::from_str(&text)?;

        // Auto-login if token is returned
        if status.is_success() {
            if let Some(token) = json.get("token").and_then(|t| t.as_str()) {
                let mut headers = self.headers.lock().await;
                if let Ok(value) = HeaderValue::from_str(token) {
                    headers.insert(AUTHORIZATION, value);
                }
                if self.verbose {
                    println!("Registered and logged in as {}", json.get("username").and_then(|u| u.as_str()).unwrap_or(""));
                }
            }
        }

        Ok(json)
    }

    /// Get MFA setup information including QR code URI.
    ///
    /// # Returns
    /// JSON with provisioning_uri, secret, and mfa_enabled status
    pub async fn get_mfa_setup(&self) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(&format!("{}/v1/user/mfa/setup", self.base_uri))
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

    /// Enable MFA for the current user.
    ///
    /// # Arguments
    /// * `mfa_token` - TOTP code from authenticator app to verify setup
    ///
    /// # Returns
    /// Response JSON with success message
    pub async fn enable_mfa(&self, mfa_token: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/user/mfa/enable", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({ "mfa_token": mfa_token }))
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

    /// Disable MFA for the current user.
    ///
    /// # Arguments
    /// * `password` - User's password (optional)
    /// * `mfa_token` - Current TOTP code (optional)
    ///
    /// # Returns
    /// Response JSON with success message
    pub async fn disable_mfa(&self, password: Option<&str>, mfa_token: Option<&str>) -> Result<serde_json::Value> {
        let mut payload = serde_json::json!({});
        if let Some(p) = password {
            payload["password"] = serde_json::json!(p);
        }
        if let Some(t) = mfa_token {
            payload["mfa_token"] = serde_json::json!(t);
        }

        let response = self
            .client
            .post(&format!("{}/v1/user/mfa/disable", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&payload)
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

    /// Change the current user's password.
    ///
    /// # Arguments
    /// * `current_password` - Current password
    /// * `new_password` - New password
    /// * `confirm_password` - New password confirmation
    ///
    /// # Returns
    /// Response JSON with success message
    pub async fn change_password(
        &self,
        current_password: &str,
        new_password: &str,
        confirm_password: &str,
    ) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/user/password/change", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "current_password": current_password,
                "new_password": new_password,
                "confirm_password": confirm_password,
            }))
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

    /// Set a password for users who don't have one (e.g., social login users).
    ///
    /// # Arguments
    /// * `new_password` - New password
    /// * `confirm_password` - New password confirmation
    ///
    /// # Returns
    /// Response JSON with success message
    pub async fn set_password(&self, new_password: &str, confirm_password: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/user/password/set", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "new_password": new_password,
                "confirm_password": confirm_password,
            }))
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

    /// Check if a user exists.
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

    /// Update user information.
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

    /// Get user information.
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

    // ==================== Chains ====================

    /// Get all chains. Returns list with chain IDs.
    pub async fn get_chains(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/chains", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let data: serde_json::Value = serde_json::from_str(&text)?;
        if let Some(arr) = data.as_array() {
            return Ok(arr.clone());
        }
        Ok(vec![])
    }

    /// Get chain ID by name. Returns None if not found.
    pub async fn get_chain_id_by_name(&self, chain_name: &str) -> Result<Option<String>> {
        let chains = self.get_chains().await?;
        for chain in chains {
            if let Some(name) = chain.get("name").and_then(|v| v.as_str()) {
                if name == chain_name {
                    return Ok(chain.get("id").and_then(|v| v.as_str()).map(String::from));
                }
            }
        }
        Ok(None)
    }

    /// Get a chain by ID.
    pub async fn get_chain(&self, chain_id: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(&format!("{}/v1/chain/{}", self.base_uri, chain_id))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let data: serde_json::Value = serde_json::from_str(&text)?;
        // Response is {chain_name: {chain_data}} - extract the chain data
        if let Some(obj) = data.as_object() {
            if obj.len() == 1 {
                if let Some(chain_data) = obj.values().next() {
                    return Ok(chain_data.clone());
                }
            }
        }
        Ok(data)
    }

    /// Get chain responses by ID.
    pub async fn get_chain_responses(&self, chain_id: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(&format!("{}/v1/chain/{}/responses", self.base_uri, chain_id))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct ChainResponse {
            chain: serde_json::Value,
        }

        let result: ChainResponse = serde_json::from_str(&text)?;
        Ok(result.chain)
    }

    /// Get chain arguments by ID.
    pub async fn get_chain_args(&self, chain_id: &str) -> Result<Vec<String>> {
        let response = self
            .client
            .get(&format!("{}/v1/chain/{}/args", self.base_uri, chain_id))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let data: Vec<String> = serde_json::from_str(&text)?;
        Ok(data)
    }

    /// Run a chain by ID.
    pub async fn run_chain(
        &self,
        chain_id: &str,
        user_input: &str,
        agent_id: Option<&str>,
        all_responses: Option<bool>,
        from_step: Option<i32>,
        chain_args: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/chain/{}/run", self.base_uri, chain_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "prompt": user_input,
                "agent_override": agent_id.unwrap_or(""),
                "all_responses": all_responses.unwrap_or(false),
                "from_step": from_step.unwrap_or(1),
                "chain_args": chain_args.unwrap_or_default(),
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

    /// Run a specific chain step by chain ID.
    pub async fn run_chain_step(
        &self,
        chain_id: &str,
        step_number: i32,
        user_input: &str,
        agent_id: Option<&str>,
        chain_args: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/chain/{}/run/step/{}", self.base_uri, chain_id, step_number))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "prompt": user_input,
                "agent_override": agent_id,
                "chain_args": chain_args.unwrap_or_default(),
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

    /// Create a new chain. Returns chain info with ID.
    pub async fn add_chain(&self, chain_name: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/chain", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({ "chain_name": chain_name }))
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Import a chain with steps.
    pub async fn import_chain(&self, chain_name: &str, steps: serde_json::Value) -> Result<String> {
        let response = self
            .client
            .post(&format!("{}/v1/chain/import", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "chain_name": chain_name,
                "steps": steps,
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

    /// Rename a chain by ID.
    pub async fn rename_chain(&self, chain_id: &str, new_name: &str) -> Result<String> {
        let response = self
            .client
            .put(&format!("{}/v1/chain/{}", self.base_uri, chain_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({ "new_name": new_name }))
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

    /// Delete a chain by ID.
    pub async fn delete_chain(&self, chain_id: &str) -> Result<String> {
        let response = self
            .client
            .delete(&format!("{}/v1/chain/{}", self.base_uri, chain_id))
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

    /// Add a step to a chain by ID.
    pub async fn add_step(
        &self,
        chain_id: &str,
        step_number: i32,
        agent_id: &str,
        prompt_type: &str,
        prompt: serde_json::Value,
    ) -> Result<String> {
        let response = self
            .client
            .post(&format!("{}/v1/chain/{}/step", self.base_uri, chain_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "step_number": step_number,
                "agent_id": agent_id,
                "prompt_type": prompt_type,
                "prompt": prompt,
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

    /// Update a chain step by chain ID.
    pub async fn update_step(
        &self,
        chain_id: &str,
        step_number: i32,
        agent_id: &str,
        prompt_type: &str,
        prompt: serde_json::Value,
    ) -> Result<String> {
        let response = self
            .client
            .put(&format!("{}/v1/chain/{}/step/{}", self.base_uri, chain_id, step_number))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "step_number": step_number,
                "agent_id": agent_id,
                "prompt_type": prompt_type,
                "prompt": prompt,
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

    /// Move a chain step by chain ID.
    pub async fn move_step(
        &self,
        chain_id: &str,
        old_step_number: i32,
        new_step_number: i32,
    ) -> Result<String> {
        let response = self
            .client
            .patch(&format!("{}/v1/chain/{}/step/move", self.base_uri, chain_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "old_step_number": old_step_number,
                "new_step_number": new_step_number,
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

    /// Delete a chain step by chain ID.
    pub async fn delete_step(&self, chain_id: &str, step_number: i32) -> Result<String> {
        let response = self
            .client
            .delete(&format!("{}/v1/chain/{}/step/{}", self.base_uri, chain_id, step_number))
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

    // ==================== Prompts ====================

    /// Create a new prompt. Returns prompt info with ID.
    pub async fn add_prompt(
        &self,
        prompt_name: &str,
        prompt: &str,
        prompt_category: Option<&str>,
    ) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/prompt", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "prompt_name": prompt_name,
                "prompt": prompt,
                "prompt_category": prompt_category.unwrap_or("Default"),
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

    /// Get a prompt by ID.
    pub async fn get_prompt(&self, prompt_id: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(&format!("{}/v1/prompt/{}", self.base_uri, prompt_id))
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

    /// Get all prompts in a category.
    pub async fn get_prompts(&self, prompt_category: Option<&str>) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/prompts", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .query(&[("prompt_category", prompt_category.unwrap_or("Default"))])
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct PromptsResponse {
            prompts: Vec<serde_json::Value>,
        }

        let result: PromptsResponse = serde_json::from_str(&text)?;
        Ok(result.prompts)
    }

    /// Get all global and user prompts with full details including IDs.
    pub async fn get_all_prompts(&self) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(&format!("{}/v1/prompt/all", self.base_uri))
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

    /// Get prompt ID by name. Returns None if not found.
    pub async fn get_prompt_id_by_name(&self, prompt_name: &str, category: Option<&str>) -> Result<Option<String>> {
        let prompts = self.get_prompts(category).await?;
        for prompt in prompts {
            if let Some(name) = prompt.get("name").and_then(|v| v.as_str()) {
                if name == prompt_name {
                    return Ok(prompt.get("id").and_then(|v| v.as_str()).map(String::from));
                }
            }
        }
        Ok(None)
    }

    /// Get all prompt categories with IDs.
    pub async fn get_prompt_categories(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/prompt/categories", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct CategoriesResponse {
            categories: Vec<serde_json::Value>,
        }

        let result: CategoriesResponse = serde_json::from_str(&text)?;
        Ok(result.categories)
    }

    /// Get prompts by category ID.
    pub async fn get_prompts_by_category_id(&self, category_id: &str) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/prompt/category/{}", self.base_uri, category_id))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct PromptsResponse {
            prompts: Vec<serde_json::Value>,
        }

        let result: PromptsResponse = serde_json::from_str(&text)?;
        Ok(result.prompts)
    }

    /// Get prompt arguments by ID.
    pub async fn get_prompt_args(&self, prompt_id: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(&format!("{}/v1/prompt/{}/args", self.base_uri, prompt_id))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct PromptArgsResponse {
            prompt_args: serde_json::Value,
        }

        let result: PromptArgsResponse = serde_json::from_str(&text)?;
        Ok(result.prompt_args)
    }

    /// Delete a prompt by ID.
    pub async fn delete_prompt(&self, prompt_id: &str) -> Result<String> {
        let response = self
            .client
            .delete(&format!("{}/v1/prompt/{}", self.base_uri, prompt_id))
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

    /// Update a prompt by ID.
    pub async fn update_prompt(&self, prompt_id: &str, prompt: &str) -> Result<String> {
        let response = self
            .client
            .put(&format!("{}/v1/prompt/{}", self.base_uri, prompt_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({ "prompt": prompt }))
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

    /// Rename a prompt by ID.
    pub async fn rename_prompt(&self, prompt_id: &str, new_name: &str) -> Result<String> {
        let response = self
            .client
            .patch(&format!("{}/v1/prompt/{}", self.base_uri, prompt_id))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({ "prompt_name": new_name }))
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

    // ==================== Companies ====================

    /// Get companies.
    pub async fn get_companies(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/companies", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let data: serde_json::Value = serde_json::from_str(&text)?;
        if let Some(arr) = data.as_array() {
            return Ok(arr.clone());
        }
        if let Some(obj) = data.as_object() {
            if let Some(companies) = obj.get("companies").and_then(|v| v.as_array()) {
                return Ok(companies.clone());
            }
        }
        Ok(vec![])
    }

    /// Get company by ID.
    pub async fn get_company(&self, company_id: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(&format!("{}/v1/company/{}", self.base_uri, company_id))
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

    // ==================== Invitations ====================

    /// Create an invitation.
    pub async fn create_invitation(&self, email: &str, role: Option<&str>) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/invitation", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "email": email,
                "role": role.unwrap_or("user"),
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

    /// Delete an invitation.
    pub async fn delete_invitation(&self, invitation_id: &str) -> Result<String> {
        let response = self
            .client
            .delete(&format!("{}/v1/invitation/{}", self.base_uri, invitation_id))
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

    // ==================== OAuth2 ====================

    /// Get OAuth2 providers.
    pub async fn get_oauth_providers(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/oauth", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let data: serde_json::Value = serde_json::from_str(&text)?;
        if let Some(arr) = data.as_array() {
            return Ok(arr.clone());
        }
        Ok(vec![])
    }

    // ==================== Text to Speech ====================

    /// Generate speech from text.
    pub async fn text_to_speech(&self, text: &str, voice: Option<&str>) -> Result<Vec<u8>> {
        let response = self
            .client
            .post(&format!("{}/v1/audio/speech", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "input": text,
                "voice": voice.unwrap_or("default"),
            }))
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let text = response.text().await?;
            return Err(crate::Error::ApiError {
                status: status.as_u16(),
                message: text,
            });
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    // ==================== Image Generation ====================

    /// Generate an image.
    pub async fn generate_image(&self, prompt: &str, n: Option<i32>) -> Result<serde_json::Value> {
        let response = self
            .client
            .post(&format!("{}/v1/images/generations", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .json(&serde_json::json!({
                "prompt": prompt,
                "n": n.unwrap_or(1),
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

    // ==================== Utility ====================

    /// Parse and log response if verbose mode is enabled.
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
