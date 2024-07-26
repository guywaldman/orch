use async_trait::async_trait;
use thiserror::Error;

use crate::lm::{
    LanguageModel, LanguageModelError, LanguageModelProvider, TextCompleteOptions,
    TextCompleteResponse, TextCompleteStreamOptions, TextCompleteStreamResponse,
};

use super::client::{
    builder::AnthropicClientBuilder, client::AnthropicClientTextCompleteOptionsBuilder,
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

        let response = client
            .text_complete(prompt, system_prompt, options)
            .await
            .map_err(|e| LanguageModelError::Anthropic(AnthropicError::Api(e.to_string())))?;

        Ok(TextCompleteResponse {
            text: response.completion,
            context: None,
        })
    }

    async fn text_complete_stream(
        &self,
        prompt: &str,
        system_prompt: &str,
        _options: TextCompleteStreamOptions,
    ) -> Result<TextCompleteStreamResponse, LanguageModelError> {
        return Err(LanguageModelError::UnsupportedFeature(
            "Streaming is not supported for Anthropic".to_string(),
        ));
    }

    async fn generate_embedding(&self, prompt: &str) -> Result<Vec<f32>, LanguageModelError> {
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
