use crate::lm::anthropic_model;

/// Default API endpoint for the Anthropic API.
pub const DEFAULT_API_ENDPOINT: &str = "https://api.anthropic.com";
/// Default model to use for text completion.
pub const DEFAULT_MODEL: &str = anthropic_model::CLAUDE_3_5_SONNET;
/// Default maximum number of tokens to generate before stopping.
pub const DEFAULT_MAX_TOKENS: usize = 2048;
