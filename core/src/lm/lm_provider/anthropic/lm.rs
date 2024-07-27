use async_trait::async_trait;
use thiserror::Error;

use crate::lm::{
    LanguageModel, LanguageModelError, LanguageModelProvider, TextCompleteOptions,
    TextCompleteResponse, TextCompleteStreamOptions, TextCompleteStreamResponse,
};

use super::client::{
    builder::AnthropicClientBuilder,
    client::AnthropicClientTextCompleteOptionsBuilder,
    models::{AnthropicMessage, AnthropicMessageRole},
};

#[derive(Debug, Clone)]
pub struct Anthropic {
    pub api_endpoint: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Error, Debug)]
pub enum AnthropicError {
    #[error("Unexpected response from API. Error: {0}")]
    Api(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error(
        "OpenAi API is not available. Please check if OpenAi is running in the specified port. Error: {0}"
    )]
    ApiUnavailable(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[async_trait]
impl LanguageModel for Anthropic {
    // TODO: Support context.
    async fn text_complete(
        &self,
        prompt: &str,
        system_prompt: &str,
        _options: TextCompleteOptions,
    ) -> Result<TextCompleteResponse, LanguageModelError> {
        let client = AnthropicClientBuilder::new()
            .with_api_endpoint(self.api_endpoint.clone())
            .with_api_key(self.api_key.clone())
            .try_build()
            .map_err(|e| {
                LanguageModelError::Anthropic(AnthropicError::Configuration(e.to_string()))
            })?;

        let options = AnthropicClientTextCompleteOptionsBuilder::new()
            .with_model(self.model.clone())
            .try_build()
            .map_err(|e| {
                LanguageModelError::Anthropic(AnthropicError::Configuration(e.to_string()))
            })?;

        // In the case of Anthropic, we need to supply the full history of the conversation.
        // We therefore parse the prompt string and construct the messages.
        let messages = Self::messages_from_prompt(prompt)?;

        let response = client
            .text_complete(messages.as_slice(), system_prompt, options)
            .await
            .map_err(|e| LanguageModelError::Anthropic(AnthropicError::Api(e.to_string())))?;

        let response_content = response
            .content
            .first()
            .ok_or(AnthropicError::Api("Response content is empty".to_string()))?;
        Ok(TextCompleteResponse {
            text: response_content.text.clone(),
            context: None,
        })
    }

    async fn text_complete_stream(
        &self,
        _prompt: &str,
        _system_prompt: &str,
        _options: TextCompleteStreamOptions,
    ) -> Result<TextCompleteStreamResponse, LanguageModelError> {
        return Err(LanguageModelError::UnsupportedFeature(
            "Streaming is not supported for Anthropic".to_string(),
        ));
    }

    async fn generate_embedding(&self, _prompt: &str) -> Result<Vec<f32>, LanguageModelError> {
        return Err(LanguageModelError::UnsupportedFeature(
				"Embedding generation is not available on Anthropic. For more details see https://docs.anthropic.com/en/docs/build-with-claude/embeddings".to_string(),
			));
    }

    fn provider(&self) -> LanguageModelProvider {
        LanguageModelProvider::Anthropic
    }

    fn text_completion_model_name(&self) -> String {
        self.model.to_string()
    }

    fn embedding_model_name(&self) -> String {
        "(UNSUPPORTED)".to_string()
    }
}

impl Anthropic {
    fn messages_from_prompt(prompt: &str) -> Result<Vec<AnthropicMessage>, LanguageModelError> {
        if !prompt.starts_with("User:") && !prompt.starts_with("Assistant:") {
            // Assume the prompt is just the user message.
            return Ok(vec![AnthropicMessage::User(prompt.to_string())]);
        }

        let mut messages = Vec::new();

        let mut iterated_prompt = prompt.to_owned();

        while !iterated_prompt.trim().is_empty() {
            let current_role = if iterated_prompt.starts_with("User:") {
                AnthropicMessageRole::User
            } else {
                AnthropicMessageRole::Assistant
            };

            let current_message = iterated_prompt
                .strip_prefix(format!("{}:", current_role).as_str())
                .map(|s| s.trim())
                .ok_or(AnthropicError::InvalidInput(
                    "Prompt is not in the expected format".to_string(),
                ))?;

            if !current_message.contains("\n\n") {
                // Last message - it contains the entire content.
                match current_role {
                    AnthropicMessageRole::User => {
                        messages.push(AnthropicMessage::User(current_message.to_owned()));
                    }
                    AnthropicMessageRole::Assistant => {
                        messages.push(AnthropicMessage::Assistant(current_message.to_owned()));
                    }
                }
                break;
            }

            // Parse until the next role.
            let (current_message, next_message) =
                current_message
                    .split_once("\n\n")
                    .ok_or(AnthropicError::InvalidInput(
                        "Prompt is not in the expected format".to_string(),
                    ))?;

            match current_role {
                AnthropicMessageRole::User => {
                    messages.push(AnthropicMessage::User(current_message.to_owned()));
                }
                AnthropicMessageRole::Assistant => {
                    messages.push(AnthropicMessage::Assistant(current_message.to_owned()));
                }
            }
            iterated_prompt = next_message.trim().to_string();
        }

        Ok(messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messages_from_prompt_single_message() {
        let prompt = "Hello";
        let messages = Anthropic::messages_from_prompt(prompt).unwrap();
        assert_eq!(messages.len(), 1);
        let Some(msg) = messages.first() else {
            panic!("Expected at least one message");
        };
        let AnthropicMessage::User(content) = msg else {
            panic!("Expected a user message");
        };
        assert_eq!(content, "Hello");
    }

    #[test]
    fn test_messages_from_prompt_multiple_messages() {
        let prompt = "User: Hello\n\nAssistant: Hi\n\nUser: How are you?";
        let messages = Anthropic::messages_from_prompt(prompt).unwrap();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0], AnthropicMessage::User("Hello".to_string()));
        assert_eq!(messages[1], AnthropicMessage::Assistant("Hi".to_string()));
        assert_eq!(
            messages[2],
            AnthropicMessage::User("How are you?".to_string())
        );
    }
}
