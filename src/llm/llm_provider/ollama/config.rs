use serde::{Deserialize, Serialize};

/// Default base URL for the Ollama API.
pub const DEFAULT_BASE_URL: &str = "http://localhost:11434";

/// Default model for text completion.
pub const DEFAULT_MODEL: &str = ollama_model::;

/// Default model for embeddings.
pub const DEFAULT_EMBEDDING_MODEL: &str = "nomic-embed-text:latest";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub base_url: Option<String>,
    pub model: Option<String>,
    pub embedding_model: Option<String>,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            base_url: Some(DEFAULT_BASE_URL.to_string()),
            model: Some("codestral:latest".to_string()),
            embedding_model: Some("nomic-embed-text:latest".to_string()),
        }
    }
}
