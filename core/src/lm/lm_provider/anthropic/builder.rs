use thiserror::Error;

use crate::lm::{LanguageModelBuilder, LanguageModelBuilderError};

use super::client::config::{DEFAULT_API_ENDPOINT, DEFAULT_MODEL};
use super::Anthropic;

#[derive(Debug, Error)]
pub enum AnthropicBuilderError {
    #[error("Configuration error: {0} is not set")]
    ConfigurationNotSet(String),
}

/// Builds an [`Anthropic`] instance.
pub struct AnthropicBuilder {
    /// API key for the Anthropic API. Required.
    api_key: Option<String>,
    /// Base URL for the Anthropic API. Defaults to [`DEFAULT_BASE_URL`].
    api_endpoint: Option<String>,
    /// Model to use for text completion. Defaults to [`DEFAULT_MODEL`].
    model: Option<String>,
}

impl AnthropicBuilder {
    /// Overrides the default base URL for the Anthropic API.
    /// Defaults to [`DEFAULT_API_ENDPOINT`].
    pub fn with_api_endpoint(mut self, base_url: String) -> Self {
        self.api_endpoint = Some(base_url);
        self
    }

    /// Sets the required API key for the Anthropic API.
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Overrides the default model to use for text completion.
    /// Defaults to [`DEFAULT_MODEL`].
    pub fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }
}

impl LanguageModelBuilder<Anthropic> for AnthropicBuilder {
    fn new() -> Self {
        Self {
            api_key: None,
            api_endpoint: Some(DEFAULT_API_ENDPOINT.to_string()),
            model: Some(DEFAULT_MODEL.to_string()),
        }
    }

    /// Tries to build an [`Anthropic`] instance. May fail if the required configurations are not set.
    fn try_build(self) -> Result<Anthropic, LanguageModelBuilderError> {
        let Some(api_endpoint) = self.api_endpoint else {
            return Err(LanguageModelBuilderError::ConfigurationNotSet(
                "API endpoint".to_string(),
            ));
        };
        let Some(api_key) = self.api_key else {
            return Err(LanguageModelBuilderError::ConfigurationNotSet(
                "API key".to_string(),
            ));
        };
        let Some(model) = self.model else {
            return Err(LanguageModelBuilderError::ConfigurationNotSet(
                "Model".to_string(),
            ));
        };
        Ok(Anthropic {
            api_key,
            api_endpoint,
            model: model.to_owned(),
        })
    }
}
