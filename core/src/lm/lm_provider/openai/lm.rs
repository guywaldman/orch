use async_trait::async_trait;
use lm::{
    error::LanguageModelError,
    models::{
        TextCompleteOptions, TextCompleteResponse, TextCompleteStreamOptions,
        TextCompleteStreamResponse,
    },
    LanguageModel, LanguageModelProvider,
};
use openai_api_rs::v1::{
    api::OpenAIClient,
    chat_completion::{self, ChatCompletionRequest},
    embedding::EmbeddingRequest,
};
use thiserror::Error;
use tokio_stream::{self as stream};

use crate::*;

#[derive(Debug, Clone)]
pub struct OpenAi {
    pub api_endpoint: Option<String>,
    pub api_key: String,
    pub model: String,
    pub embeddings_model: String,
}

#[derive(Error, Debug)]
pub enum OpenAiError {
    #[error("Unexpected response from API. Error: {0}")]
    Api(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("OpenAI API is not available. Error: {0}")]
    ApiUnavailable(String),
}

#[async_trait]
impl LanguageModel for OpenAi {
    async fn text_complete(
        &self,
        prompt: &str,
        system_prompt: &str,
        _options: TextCompleteOptions,
    ) -> Result<TextCompleteResponse, LanguageModelError> {
        let mut client = OpenAIClient::new(self.api_key.to_owned());

        if let Some(api_endpoint) = self.api_endpoint.clone() {
            client.api_endpoint = api_endpoint;
        }

        let messages = vec![
            chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::system,
                content: chat_completion::Content::Text(system_prompt.to_owned()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(prompt.to_owned()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ];
        // TODO: Support customization of max tokens and temperature.
        let req = ChatCompletionRequest::new(self.model.to_owned(), messages);

        let result = client
            .chat_completion(req)
            .await
            .map_err(|e| LanguageModelError::OpenAi(OpenAiError::Api(e.to_string())))?;
        let completion = result
            .choices
            .first()
            .unwrap()
            .message
            .content
            .clone()
            .unwrap();
        Ok(TextCompleteResponse {
            text: completion,
            // TODO: Support context.
            context: None,
        })
    }

    async fn text_complete_stream(
        &self,
        prompt: &str,
        system_prompt: &str,
        _options: TextCompleteStreamOptions,
    ) -> Result<TextCompleteStreamResponse, LanguageModelError> {
        // TODO: Support streaming - currently it just sends a single message.
        let text_completion_response = self
            .text_complete(prompt, system_prompt, TextCompleteOptions { context: None })
            .await?;
        Ok(TextCompleteStreamResponse {
            stream: Box::pin(stream::once(Ok(text_completion_response.text))),
        })
    }

    async fn generate_embedding(&self, prompt: &str) -> Result<Vec<f32>, LanguageModelError> {
        let client = OpenAIClient::new(self.api_key.to_owned());

        let resp = client
            .embedding(EmbeddingRequest {
                model: self.embeddings_model.to_owned(),
                input: prompt.to_owned(),
                dimensions: None,
                user: None,
            })
            .await
            .map_err(|e| LanguageModelError::OpenAi(OpenAiError::Api(e.to_string())))?;

        let data = resp.data.first().expect("Embedding data not found");
        Ok(data.embedding.clone())
    }

    fn provider(&self) -> LanguageModelProvider {
        LanguageModelProvider::OpenAi
    }

    fn text_completion_model_name(&self) -> String {
        self.model.to_string()
    }

    fn embedding_model_name(&self) -> String {
        self.embeddings_model.to_string()
    }
}
