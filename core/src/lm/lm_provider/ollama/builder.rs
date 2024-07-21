use thiserror::Error;

use crate::lm::{LanguageModelBuilder, LanguageModelBuilderError};

use super::{ollama_embedding_model, ollama_model, Ollama};

#[derive(Debug, Error)]
pub enum OllamaBuilderError {
    #[error("Configuration error: {0} is not set")]
    ConfigurationNotSet(String),
}

pub struct OllamaBuilder {
    base_url: Option<String>,
    model: Option<String>,
    embeddings_model: Option<String>,
}

impl OllamaBuilder {
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
            base_url: Some("http://localhost:11434".to_string()),
            model: Some(ollama_model::CODESTRAL.to_string()),
            embeddings_model: Some(ollama_embedding_model::NOMIC_EMBED_TEXT.to_string()),
        }
    }

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
