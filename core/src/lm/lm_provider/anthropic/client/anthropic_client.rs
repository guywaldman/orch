#![allow(dead_code)]

use thiserror::Error;

use crate::lm::lm_provider::anthropic::client::models::AnthropicMessagesApiRequest;

use super::{
    config::DEFAULT_MAX_TOKENS,
    models::{
        AnthropicMessage, AnthropicMessagesApiMessage, AnthropicMessagesApiResponse,
        AnthropicMessagesApiResponseSuccess,
    },
};

#[derive(Debug, Error)]
pub(crate) enum AnthropicClientError {
    #[error("{0}")]
    InternalError(String),

    #[error("Configuration '{0}' is not set")]
    ConfigurationNotSet(String),

    #[error("Failed to serialize/deserialize: {0}")]
    Marhsalling(String),

    #[error("Failed to send or receive request to/from Anthropic API: {0}")]
    Api(String),
}

/// A client for interacting with the Anthropic API.
pub struct AnthropicClient {
    pub(crate) api_endpoint: String,
    pub(crate) api_key: String,
}

impl AnthropicClient {
    /// Generates a response from the Anthropic API.
    ///
    /// # Arguments
    /// * `prompt` - The prompt to generate a response for. Use "User:...\n\n" for user messages and "Assistant:...\n\n" for assistant messages.
    /// * `system_prompt` - The system prompt to use for the generation.
    /// * `options` - The options for the generation (use [`AnthropicClientTextCompleteOptionsBuilder`] to build a new instance).
    ///
    /// # Returns
    /// A [Result] containing the response from the Anthropic API or an error if there was a problem.
    pub async fn text_complete(
        &self,
        messages: &[AnthropicMessage],
        system_prompt: &str,
        options: AnthropicClientTextCompleteOptions,
    ) -> Result<AnthropicMessagesApiResponseSuccess, AnthropicClientError> {
        let messages_api_endpoint = format!("{}/v1/messages", self.api_endpoint);

        let system_prompt = if system_prompt.is_empty() {
            None
        } else {
            Some(system_prompt.to_string())
        };

        let messages = messages
            .iter()
            .map(Self::construct_message)
            .collect::<Vec<_>>();

        let req_body = AnthropicMessagesApiRequest {
            messages,
            system_prompt,
            model: options.model,
            max_tokens_to_sample: DEFAULT_MAX_TOKENS,
            stop_sequences: None,
            temperature: None,
            top_k: None,
        };

        let http_client = reqwest::Client::new();
        let req = http_client
            .post(messages_api_endpoint)
            // See Anthropic authentication documentation: https://docs.anthropic.com/en/api/getting-started#authentication
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(
                serde_json::to_string(&req_body)
                    .map_err(|e| AnthropicClientError::Marhsalling(e.to_string()))?,
            )
            .build()
            .map_err(|e| AnthropicClientError::InternalError(e.to_string()))?;

        let response = http_client
            .execute(req)
            .await
            .map_err(|e| AnthropicClientError::Api(e.to_string()))?;

        let response_body_json = response
            .text()
            .await
            .map_err(|e| AnthropicClientError::Api(e.to_string()))?
            .to_string();

        let deserialized_response: AnthropicMessagesApiResponse =
            serde_json::from_str(&response_body_json).map_err(|e| {
                AnthropicClientError::Marhsalling(format!(
                    "Failed to parse response: {e} (response: {response_body_json})"
                ))
            })?;

        Ok(match deserialized_response {
            AnthropicMessagesApiResponse::Success(success_response) => success_response,
            AnthropicMessagesApiResponse::Error(error_response) => {
                let error_message = error_response.error.message;
                return Err(AnthropicClientError::Api(error_message));
            }
        })
    }

    fn construct_message(msg: &AnthropicMessage) -> AnthropicMessagesApiMessage {
        match msg {
            AnthropicMessage::User(content) => AnthropicMessagesApiMessage {
                role: "user".to_string(),
                content: content.to_string(),
            },
            AnthropicMessage::Assistant(content) => AnthropicMessagesApiMessage {
                role: "assistant".to_string(),
                content: content.to_string(),
            },
        }
    }
}

/// Options for text completion.
#[derive(Debug, Default)]
pub struct AnthropicClientTextCompleteOptions {
    /// See [`AnthropicCompleteApiRequest::model`].
    pub model: String,
    /// See [`AnthropicCompleteApiRequest::max_tokens`].
    pub max_tokens_to_sample: usize,
    /// See [`AnthropicCompleteApiRequest::stop_sequences`].
    pub stop_sequences: Option<Vec<String>>,
    /// See [`AnthropicCompleteApiRequest::temperature`].
    pub temperature: Option<f32>,
    /// See [`AnthropicCompleteApiRequest::top_k`].
    pub top_k: Option<usize>,
}

/// Builds a new [`AnthropicClientTextCompleteOptions`] instance.
#[derive(Debug, Default)]
pub struct AnthropicClientTextCompleteOptionsBuilder {
    model: Option<String>,
    max_tokens: usize,
    stop_sequences: Option<Vec<String>>,
    temperature: Option<f32>,
    top_k: Option<usize>,
}

impl AnthropicClientTextCompleteOptionsBuilder {
    pub fn new() -> Self {
        Self {
            max_tokens: DEFAULT_MAX_TOKENS,
            ..Default::default()
        }
    }

    /// Sets the model (required).
    pub fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    /// Sets the maximum number of tokens to generate before stopping.
    /// Defaults to [`DEFAULT_MAX_TOKENS`].
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Sets the stop sequences.
    pub fn with_stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }

    /// Sets the temperature.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the top k.
    pub fn with_top_k(mut self, top_k: usize) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Tries to build a [`AnthropicClientTextCompleteOptions`] instance. May fail if the required configurations are not set.
    pub fn try_build(self) -> Result<AnthropicClientTextCompleteOptions, AnthropicClientError> {
        let Some(model) = self.model else {
            return Err(AnthropicClientError::ConfigurationNotSet(
                "Model".to_string(),
            ));
        };
        Ok(AnthropicClientTextCompleteOptions {
            model,
            max_tokens_to_sample: self.max_tokens,
            stop_sequences: self.stop_sequences,
            temperature: self.temperature,
            top_k: self.top_k,
        })
    }
}
