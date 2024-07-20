use thiserror::Error;

use super::{LanguageModelProvider, OllamaError};

#[derive(Debug, Error)]
pub enum LanguageModelProviderError {
    #[error("Invalid LLM provider: {0}")]
    InvalidValue(String),
}

impl std::fmt::Display for LanguageModelProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageModelProvider::Ollama => write!(f, "ollama"),
            LanguageModelProvider::OpenAi => write!(f, "openai"),
        }
    }
}

impl Default for LanguageModelProvider {
    fn default() -> Self {
        Self::Ollama
    }
}

impl TryFrom<&str> for LanguageModelProvider {
    type Error = LanguageModelProviderError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ollama" => Ok(LanguageModelProvider::Ollama),
            "openai" => Ok(LanguageModelProvider::OpenAi),
            _ => Err(LanguageModelProviderError::InvalidValue(value.to_string())),
        }
    }
}

#[derive(Debug, Error)]
pub enum LanguageModelError {
    #[error("Text generation error: {0}")]
    TextGeneration(String),

    #[error("Embedding generation error: {0}")]
    EmbeddingGeneration(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Ollama error: {0}")]
    Ollama(#[from] OllamaError),
}
