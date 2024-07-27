use thiserror::Error;

use crate::lm::{lm_provider::openai::config, LanguageModelBuilder, LanguageModelBuilderError};

use super::OpenAi;

#[derive(Debug, Error)]
pub enum OpenAiBuilderError {
    #[error("Configuration error: {0} is not set")]
    ConfigurationNotSet(String),
}

/// Builds an [`OpenAi`] instance.
pub struct OpenAiBuilder {
    api_endpoint: Option<String>,
    api_key: Option<String>,
    model: Option<String>,
    embeddings_model: Option<String>,
}

impl OpenAiBuilder {
    /// Sets the required API key for the OpenAI API.
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Overrides the default API endpoint for the OpenAI API.
    pub fn with_api_endpoint(mut self, api_endpoint: String) -> Self {
        self.api_endpoint = Some(api_endpoint);
        self
    }

    /// Sets the model to use for text completion. Defaults to [`config::DEFAULT_MODEL`].
    pub fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    /// Sets the model to use for embedding generation. Defaults to [`config::DEFAULT_EMBEDDINGS_MODEL`].
    pub fn with_embeddings_model(mut self, embeddings_model: String) -> Self {
        self.embeddings_model = Some(embeddings_model.clone());
        self
    }
}

impl LanguageModelBuilder<OpenAi> for OpenAiBuilder {
    fn new() -> Self {
        Self {
            api_key: None,
            api_endpoint: None,
            model: Some(config::DEFAULT_MODEL.to_string()),
            embeddings_model: Some(config::DEFAULT_EMBEDDINGS_MODEL.to_string()),
        }
    }

    /// Tries to build an [`OpenAi`] instance. May fail if the required configurations are not set.
    fn try_build(self) -> Result<OpenAi, LanguageModelBuilderError> {
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
        let Some(embeddings_model) = self.embeddings_model else {
            return Err(LanguageModelBuilderError::ConfigurationNotSet(
                "Embeddings model".to_string(),
            ));
        };
        Ok(OpenAi {
            api_endpoint: self.api_endpoint,
            api_key: api_key.to_owned(),
            model: model.to_owned(),
            embeddings_model: embeddings_model.to_owned(),
        })
    }
}
