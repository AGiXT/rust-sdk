use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderResponse {
    pub providers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderSettings {
    pub settings: HashMap<String, serde_json::Value>,
}

impl super::AGiXTSDK {
    /// Get list of available providers
    pub async fn get_providers(&self) -> Result<Vec<String>> {
        let response = self
            .client
            .get(&format!("{}/api/provider", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let result: ProviderResponse = serde_json::from_str(&text)?;
        Ok(result.providers)
    }

    /// Get providers by service type
    pub async fn get_providers_by_service(&self, service: &str) -> Result<Vec<String>> {
        let response = self
            .client
            .get(&format!("{}/api/providers/service/{}", self.base_uri, service))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let result: ProviderResponse = serde_json::from_str(&text)?;
        Ok(result.providers)
    }

    /// Get settings for a specific provider
    pub async fn get_provider_settings(&self, provider_name: &str) -> Result<HashMap<String, serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/api/provider/{}", self.base_uri, provider_name))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let result: ProviderSettings = serde_json::from_str(&text)?;
        Ok(result.settings)
    }

    /// Get list of embedding providers
    pub async fn get_embed_providers(&self) -> Result<Vec<String>> {
        let response = self
            .client
            .get(&format!("{}/api/embedding_providers", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        let result: ProviderResponse = serde_json::from_str(&text)?;
        Ok(result.providers)
    }

    /// Get details of all embedders
    pub async fn get_embedders(&self) -> Result<HashMap<String, serde_json::Value>> {
        let response = self
            .client
            .get(&format!("{}/api/embedders", self.base_uri))
            .headers(self.headers.lock().await.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if self.verbose {
            self.parse_response(status, &text).await?;
        }

        #[derive(Deserialize)]
        struct EmbeddersResponse {
            embedders: HashMap<String, serde_json::Value>,
        }

        let result: EmbeddersResponse = serde_json::from_str(&text)?;
        Ok(result.embedders)
    }
}

#[cfg(test)]
mod tests {
    use crate::AGiXTSDK;
    use mockito;

    #[tokio::test]
    async fn test_get_providers() {
        let mut mock_server = mockito::Server::new();
        let _mock = mock_server
            .mock("GET", "/api/provider")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"providers": ["provider1", "provider2"]}"#)
            .create();

        let client = AGiXTSDK::new(Some(mock_server.url()), None, false);
        let providers = client.get_providers().await.unwrap();
        
        assert_eq!(providers, vec!["provider1", "provider2"]);
    }
}