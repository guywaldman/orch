use thiserror::Error;

use super::{anthropic_client::AnthropicClient, config};

#[derive(Debug, Error)]
pub enum AnthropicBuilderError {
    #[error("Configuration error: {0} is not set")]
    ConfigurationNotSet(String),
}

/// Builds an [`AnthropicClient`] instance.
pub struct AnthropicClientBuilder {
    api_endpoint: String,
    api_key: Option<String>,
}

impl AnthropicClientBuilder {
    pub fn new() -> Self {
        Self {
            api_endpoint: config::DEFAULT_API_ENDPOINT.to_string(),
            api_key: None,
        }
    }

    /// Sets an override for the Anthropic API endpoint. Defaults to [`config::DEFAULT_API_ENDPOINT`].
    pub fn with_api_endpoint(mut self, api_endpoint: String) -> Self {
        self.api_endpoint = api_endpoint;
        self
    }

    /// Sets the required Anthropic API key.
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn try_build(self) -> Result<AnthropicClient, AnthropicBuilderError> {
        let Some(api_key) = self.api_key else {
            return Err(AnthropicBuilderError::ConfigurationNotSet(
                "API key".to_string(),
            ));
        };
        Ok(AnthropicClient {
            api_endpoint: self.api_endpoint,
            api_key,
        })
    }
}
