use thiserror::Error;

use crate::lm::{LanguageModelBuilder, LanguageModelBuilderError};

use super::{openai_embedding_model, openai_model, OpenAi};

#[derive(Debug, Error)]
pub enum OpenAiBuilderError {
    #[error("Configuration error: {0} is not set")]
    ConfigurationNotSet(String),
}

pub struct OpenAiBuilder {
    api_key: Option<String>,
    model: Option<String>,
    embeddings_model: Option<String>,
    embedding_dimensions: Option<usize>,
}

impl OpenAiBuilder {
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    pub fn with_embeddings_model(mut self, embeddings_model: String) -> Self {
        self.embeddings_model = Some(embeddings_model.clone());
        self.embedding_dimensions = match embeddings_model.as_ref() {
            openai_embedding_model::TEXT_EMBEDDING_ADA_002 => {
                Some(openai_embedding_model::TEXT_EMBEDDING_ADA_002_DIMENSIONS)
            }
            openai_embedding_model::TEXT_EMBEDDING_3_SMALL => {
                Some(openai_embedding_model::TEXT_EMBEDDING_3_SMALL_DIMENSIONS)
            }
            openai_embedding_model::TEXT_EMBEDDING_3_LARGE => {
                Some(openai_embedding_model::TEXT_EMBEDDING_3_LARGE_DIMENSIONS)
            }
            _ => None,
        };
        self
    }
}

impl LanguageModelBuilder<OpenAi> for OpenAiBuilder {
    fn new() -> Self {
        Self {
            api_key: None,
            model: Some(openai_model::GPT_4O_MINI.to_string()),
            embeddings_model: Some(openai_embedding_model::TEXT_EMBEDDING_ADA_002.to_string()),
            embedding_dimensions: Some(openai_embedding_model::TEXT_EMBEDDING_ADA_002_DIMENSIONS),
        }
    }

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
            api_key: api_key.to_owned(),
            model: model.to_owned(),
            embeddings_model: embeddings_model.to_owned(),
        })
    }
}
