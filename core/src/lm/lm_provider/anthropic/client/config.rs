use super::models::anthropic_model;

/// Default API endpoint for the Anthropic API.
pub const DEFAULT_API_ENDPOINT: &str = "https://api.anthropic.com";
/// Default model to use for text completion.
pub const DEFAULT_MODEL: &str = anthropic_model::CLAUDE_3_5_SONNET;
