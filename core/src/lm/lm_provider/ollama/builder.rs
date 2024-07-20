use thiserror::Error;

use crate::lm::{LanguageModelBuilder, LanguageModelBuilderError};

use super::{ollama_embedding_model, ollama_model, Ollama};

#[derive(Debug, Error)]
pub enum OllamaBuilderError {
    #[error("Configuration error: {0} is not set")]
    ConfigurationNotSet(String),
}

pub struct OllamaBuilder<'a> {
    base_url: Option<&'a str>,
    model: Option<&'a str>,
    embeddings_model: Option<&'a str>,
}

impl<'a> OllamaBuilder<'a> {
    pub fn with_base_url(mut self, base_url: &'a str) -> Self {
        self.base_url = Some(base_url);
        self
    }

    pub fn with_model(mut self, model: &'a str) -> Self {
        self.model = Some(model);
        self
    }

    pub fn with_embeddings_model(mut self, embeddings_model: &'a str) -> Self {
        self.embeddings_model = Some(embeddings_model);
        self
    }
}

impl<'a> LanguageModelBuilder<Ollama<'a>> for OllamaBuilder<'a> {
    fn new() -> Self {
        Self {
            base_url: Some("http://localhost:11434"),
            model: Some(ollama_model::CODESTRAL),
            embeddings_model: Some(ollama_embedding_model::NOMIC_EMBED_TEXT),
        }
    }

    fn try_build(self) -> Result<Ollama<'a>, LanguageModelBuilderError> {
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
            base_url,
            model,
            embeddings_model,
        })
    }
}
