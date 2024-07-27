use super::{openai_embedding_model, openai_model};

/// Default model to use for text completion.
pub const DEFAULT_MODEL: &str = openai_model::GPT_4O_MINI;
/// Default model to use for embedding generation.
pub const DEFAULT_EMBEDDINGS_MODEL: &str = openai_embedding_model::TEXT_EMBEDDING_ADA_002;
