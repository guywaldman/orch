use thiserror::Error;

use crate::lm::{LanguageModelBuilder, LanguageModelBuilderError};

use super::config::{DEFAULT_BASE_URL, DEFAULT_EMBEDDINGS_MODEL, DEFAULT_MODEL};
use super::Ollama;

#[derive(Debug, Error)]
pub enum OllamaBuilderError {
    #[error("Configuration error: {0} is not set")]
    ConfigurationNotSet(String),
}

/// Builds an [`Ollama`] instance.
pub struct OllamaBuilder {
    /// Base URL for the Ollama API. Defaults to [`config::DEFAULT_BASE_URL`].
    base_url: Option<String>,
    /// Model to use for text completion. Defaults to [`config::DEFAULT_MODEL`].
    model: Option<String>,
    /// Model to use for embedding generation. Defaults to [`config::DEFAULT_EMBEDDINGS_MODEL`].
    embeddings_model: Option<String>,
}

impl OllamaBuilder {
    /// Overrides the default base URL for the Ollama API.
    /// Defaults to
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    pub fn with_embeddings_model(mut self, embeddings_model: String) -> Self {
        self.embeddings_model = Some(embeddings_model);
        self
    }
}

impl LanguageModelBuilder<Ollama> for OllamaBuilder {
    fn new() -> Self {
        Self {
            base_url: Some(DEFAULT_BASE_URL.to_string()),
            model: Some(DEFAULT_MODEL.to_string()),
            embeddings_model: Some(DEFAULT_EMBEDDINGS_MODEL.to_string()),
        }
    }

    /// Tries to build an [`Ollama`] instance. May fail if the required configurations are not set.
    fn try_build(self) -> Result<Ollama, LanguageModelBuilderError> {
        let Some(base_url) = self.base_url else {
            return Err(LanguageModelBuilderError::ConfigurationNotSet(
                "Base URL".to_string(),
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
        Ok(Ollama {
            base_url: base_url.to_owned(),
            model: model.to_owned(),
            embeddings_model: embeddings_model.to_owned(),
        })
    }
}
