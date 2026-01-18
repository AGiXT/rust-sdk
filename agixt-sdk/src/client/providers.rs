//! Provider operations using /v1 endpoints.

use crate::error::Result;
use std::collections::HashMap;

impl super::AGiXTSDK {
    // ==================== Providers ====================

    /// Get list of available providers.
    pub async fn get_providers(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/provider", self.base_uri))
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
            if let Some(providers) = obj.get("providers").and_then(|v| v.as_array()) {
                return Ok(providers.clone());
            }
        }
        Ok(vec![])
    }

    /// Get providers by service type.
    pub async fn get_providers_by_service(&self, service: &str) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/providers/service/{}", self.base_uri, service))
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
            if let Some(providers) = obj.get("providers").and_then(|v| v.as_array()) {
                return Ok(providers.clone());
            }
        }
        Ok(vec![])
    }

    /// Get settings for a specific provider.
    pub async fn get_provider_settings(&self, provider_name: &str) -> Result<HashMap<String, serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/provider/{}", self.base_uri, provider_name))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct SettingsResponse {
            settings: HashMap<String, serde_json::Value>,
        }

        let result: SettingsResponse = serde_json::from_str(&text)?;
        Ok(result.settings)
    }

    /// Get list of embedding providers.
    pub async fn get_embed_providers(&self) -> Result<Vec<String>> {
        let providers = self.get_providers().await?;
        let mut embed_providers = Vec::new();
        
        for provider in providers {
            if let Some(obj) = provider.as_object() {
                if obj.get("supports_embeddings").and_then(|v| v.as_bool()).unwrap_or(false) {
                    if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                        embed_providers.push(name.to_string());
                    }
                }
            }
        }
        
        Ok(embed_providers)
    }

    /// Get details of all embedders.
    pub async fn get_embedders(&self) -> Result<HashMap<String, serde_json::Value>> {
        let providers = self.get_providers().await?;
        let mut embedders = HashMap::new();
        
        for provider in providers {
            if let Some(obj) = provider.as_object() {
                if obj.get("supports_embeddings").and_then(|v| v.as_bool()).unwrap_or(false) {
                    if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                        embedders.insert(name.to_string(), provider.clone());
                    }
                }
            }
        }
        
        Ok(embedders)
    }

    // ==================== Extensions ====================

    /// Get extension settings.
    pub async fn get_extension_settings(&self) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(&format!("{}/v1/extensions/settings", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct ExtensionSettingsResponse {
            extension_settings: serde_json::Value,
        }

        let result: ExtensionSettingsResponse = serde_json::from_str(&text)?;
        Ok(result.extension_settings)
    }

    /// Get all available extensions.
    pub async fn get_extensions(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/v1/extensions", self.base_uri))
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
            if let Some(extensions) = obj.get("extensions").and_then(|v| v.as_array()) {
                return Ok(extensions.clone());
            }
        }
        Ok(vec![])
    }

    /// Get arguments for a command.
    pub async fn get_command_args(&self, command_name: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(&format!("{}/v1/extensions/{}/args", self.base_uri, command_name))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(serde::Deserialize)]
        struct CommandArgsResponse {
            command_args: serde_json::Value,
        }

        let result: CommandArgsResponse = serde_json::from_str(&text)?;
        Ok(result.command_args)
    }
}
