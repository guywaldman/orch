use thiserror::Error;

use crate::lm::lm_provider::anthropic::client::models::AnthropicCompleteApiRequest;

use super::models::AnthropicCompleteApiResponse;

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
    /// * `prompt` - The prompt to generate a response for.
    /// * `system_prompt` - The system prompt to use for the generation.
    /// * `options` - The options for the generation (use [`AnthropicClientTextCompleteOptionsBuilder`] to build a new instance).
    ///
    /// # Returns
    /// A [Result] containing the response from the Anthropic API or an error if there was a problem.
    pub async fn text_complete(
        &self,
        prompt: &str,
        system_prompt: &str,
        options: AnthropicClientTextCompleteOptions,
    ) -> Result<AnthropicCompleteApiResponse, AnthropicClientError> {
        let messages_api_endpoint = format!("{}/v1/complete", self.api_endpoint);

        let prompt = format!("Assistant: {system_prompt}\n\nHuman: {prompt}");

        let req_body = AnthropicCompleteApiRequest {
            prompt,
            model: options.model,
            max_tokens: None,
            stop_sequences: None,
            temperature: None,
            top_k: None,
        };

        let http_client = reqwest::Client::new();
        let req = http_client
            .post(messages_api_endpoint)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.api_key),
            )
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

        let deserialized_response: AnthropicCompleteApiResponse =
            serde_json::from_str(&response_body_json)
                .map_err(|e| AnthropicClientError::Marhsalling(e.to_string()))?;

        Ok(deserialized_response)
    }
}

/// Options for text completion.
#[derive(Debug, Default)]
pub struct AnthropicClientTextCompleteOptions {
    /// See [`AnthropicCompleteApiRequest::model`].
    pub model: String,
    /// See [`AnthropicCompleteApiRequest::max_tokens`].
    pub max_tokens: Option<usize>,
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
    max_tokens: Option<usize>,
    stop_sequences: Option<Vec<String>>,
    temperature: Option<f32>,
    top_k: Option<usize>,
}

impl AnthropicClientTextCompleteOptionsBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the model (required).
    pub fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    /// Sets the maximum number of tokens to generate before stopping.
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = Some(max_tokens);
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
            max_tokens: self.max_tokens,
            stop_sequences: self.stop_sequences,
            temperature: self.temperature,
            top_k: self.top_k,
        })
    }
}
