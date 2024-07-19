use thiserror::Error;

use crate::{LlmProvider, OllamaError};

#[derive(Debug, Error)]
pub enum LlmProviderError {
    #[error("Invalid LLM provider: {0}")]
    InvalidValue(String),
}

impl std::fmt::Display for LlmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmProvider::Ollama => write!(f, "ollama"),
            LlmProvider::OpenAi => write!(f, "openai"),
        }
    }
}

impl Default for LlmProvider {
    fn default() -> Self {
        Self::Ollama
    }
}

impl TryFrom<&str> for LlmProvider {
    type Error = LlmProviderError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ollama" => Ok(LlmProvider::Ollama),
            "openai" => Ok(LlmProvider::OpenAi),
            _ => Err(LlmProviderError::InvalidValue(value.to_string())),
        }
    }
}

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("Text generation error: {0}")]
    TextGeneration(String),

    #[error("Embedding generation error: {0}")]
    EmbeddingGeneration(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Ollama error: {0}")]
    Ollama(#[from] OllamaError),
}
