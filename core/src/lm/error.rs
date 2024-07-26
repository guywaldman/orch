use thiserror::Error;

use super::{AnthropicError, LanguageModelProvider, OllamaError, OpenAiError};

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
            LanguageModelProvider::Anthropic => write!(f, "anthropic"),
        }
    }
}

impl Default for LanguageModelProvider {
    fn default() -> Self {
        Self::Ollama
    }
}

#[derive(Debug, Error)]
pub enum LanguageModelError {
    #[error("Text generation error: {0}")]
    TextGeneration(String),

    #[error("Feature unsupported: {0}")]
    UnsupportedFeature(String),

    #[error("Embedding generation error: {0}")]
    EmbeddingGeneration(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Ollama error: {0}")]
    Ollama(#[from] OllamaError),

    #[error("OpenAI error: {0}")]
    OpenAi(#[from] OpenAiError),

    #[error("Anthropic error: {0}")]
    Anthropic(#[from] AnthropicError),
}
