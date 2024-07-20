use thiserror::Error;

use crate::lm::{LanguageModelBuilder, LanguageModelBuilderError};

use super::{openai_embedding_model, OpenAi};

#[derive(Debug, Error)]
pub enum OpenAiBuilderError {
    #[error("Configuration error: {0} is not set")]
    ConfigurationNotSet(String),
}

pub struct OpenAiBuilder<'a> {
    api_key: Option<&'a str>,
    model: Option<&'a str>,
    embeddings_model: Option<&'a str>,
    embedding_dimensions: Option<usize>,
}

impl Default for OpenAiBuilder<'_> {
    fn default() -> Self {
        Self {
            api_key: None,
            model: None,
            embeddings_model: None,
            embedding_dimensions: None,
        }
    }
}

impl<'a> OpenAiBuilder<'a> {
    pub fn with_api_key(mut self, api_key: &'a str) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn with_model(mut self, model: &'a str) -> Self {
        self.model = Some(model);
        self
    }

    pub fn with_embeddings_model(mut self, embeddings_model: &'a str) -> Self {
        self.embeddings_model = Some(embeddings_model);
        match embeddings_model {
            openai_embedding_model::TEXT_EMBEDDING_ADA_002 => {
                self.embedding_dimensions =
                    Some(openai_embedding_model::TEXT_EMBEDDING_ADA_002_DIMENSIONS);
            }
            openai_embedding_model::TEXT_EMBEDDING_3_SMALL => {
                self.embedding_dimensions =
                    Some(openai_embedding_model::TEXT_EMBEDDING_3_SMALL_DIMENSIONS);
            }
            openai_embedding_model::TEXT_EMBEDDING_3_LARGE => {
                self.embedding_dimensions =
                    Some(openai_embedding_model::TEXT_EMBEDDING_3_LARGE_DIMENSIONS);
            }
            _ => {}
        }
        self
    }
}

impl<'a> LanguageModelBuilder<OpenAi<'a>> for OpenAiBuilder<'a> {
    fn new() -> Self {
        Default::default()
    }

    fn try_build(self) -> Result<OpenAi<'a>, LanguageModelBuilderError> {
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
        let Some(embedding_dimensions) = self.embedding_dimensions else {
            return Err(LanguageModelBuilderError::ConfigurationNotSet(
                "Embedding dimensions".to_string(),
            ));
        };
        Ok(OpenAi {
            api_key,
            model,
            embeddings_model,
            embedding_dimensions,
        })
    }
}
