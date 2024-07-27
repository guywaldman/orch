use super::{ollama_embedding_model, ollama_model};

/// Default base URL for the Ollama API.
pub const DEFAULT_BASE_URL: &str = "http://localhost:11434";
/// Default model to use for text completion.
pub const DEFAULT_MODEL: &str = ollama_model::LLAMA3_1_8B;
/// Default model to use for embedding generation.
pub const DEFAULT_EMBEDDINGS_MODEL: &str = ollama_embedding_model::NOMIC_EMBED_TEXT;
