use serde::{Deserialize, Serialize};

/// Request for generating a response from the Anthropic API.
/// Referenced from the Anthropic API documentation [here](https://docs.anthropic.com/en/api/complete).
#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicMessagesApiRequest {
    /// The model that will complete your prompt.
    /// See [`anthropic_model`] for a list of built-in model IDs for convenience.
    ///
    /// See [models](https://docs.anthropic.com/en/docs/about-claude/models) for a complete list of models supported by Anthropic.
    pub model: String,

    /// Messages to generate a completion for.
    ///
    /// See [Anthropic API documentation](https://docs.anthropic.com/en/api/messages) for more information.
    pub messages: Vec<AnthropicMessagesApiMessage>,

    /// Optional system prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "system")]
    pub system_prompt: Option<String>,

    /// The maximum number of tokens to generate before stopping.
    ///
    /// Note that the Anthropic models may stop before reaching this maximum. This parameter only specifies the absolute maximum number of tokens to generate.
    #[serde(rename = "max_tokens")]
    pub max_tokens_to_sample: usize,

    /// Sequences that will cause the model to stop generating.
    /// The Anthropic models stop on "\n\nHuman:", and may include additional built-in stop sequences in the future.
    /// By providing the stop_sequences parameter, you may include additional strings that will cause the model to stop generating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,

    /// Amount of randomness injected into the response.
    ///
    /// Defaults to 1.0. Ranges from 0.0 to 1.0. Use temperature closer to 0.0 for analytical / multiple choice, and closer to 1.0 for creative and generative tasks.
    ///
    // Note that even with temperature of 0.0, the results will not be fully deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Only sample from the top K options for each subsequent token.
    ///
    /// Used to remove "long tail" low probability responses. Learn more technical details [here](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277).
    ///
    /// Recommended for advanced use cases only. You usually only need to use temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AnthropicMessage {
    /// A user message.
    User(String),
    /// An assistant message.
    Assistant(String),
}

#[derive(Debug)]
pub enum AnthropicMessageRole {
    User,
    Assistant,
}

impl std::fmt::Display for AnthropicMessageRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnthropicMessageRole::User => write!(f, "User"),
            AnthropicMessageRole::Assistant => write!(f, "Assistant"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicMessagesApiMessage {
    /// The role of the message.
    /// For a user, this will be "user". For an assistant, this will be "assistant".
    pub role: String,

    /// The content of the message.
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnthropicMessagesApiResponse {
    Success(AnthropicMessagesApiResponseSuccess),
    Error(AnthropicApiError),
}

/// Response from the Anthropic API for generating a response.
/// Referenced from the Anthropic API documentation [here](https://docs.anthropic.com/en/api/complete).
#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicMessagesApiResponseSuccess {
    /// Object type. For text completion, this will be "completion".
    #[serde(rename = "type")]
    pub typ: String,

    /// The role of the message. For responses, this would be "assistant".
    pub role: String,

    /// The model that generated the response.
    pub model: String,

    /// The resulting completion up to and excluding the stop sequences.
    pub content: Vec<AnthropicMessagesApiResponseSuccessContent>,

    /// The reason that the model stopped generating tokens.
    ///
    /// This may be one the following values:
    /// - "stop_sequence": Reached a stop sequence — either provided by you via the stop_sequences parameter, or a stop sequence built into the model
    /// - "max_tokens": Exceeded `max_tokens_to_sample` or the model's maximum
    pub stop_reason: Option<String>,
}

/// Response from the Anthropic API for the messages API endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicMessagesApiResponseSuccessContent {
    /// Type of the response, for text completion, this will be "text".
    #[serde(rename = "type")]
    pub typ: String,

    /// The content of the response.
    pub text: String,
}

/// Response from the Anthropic API which indicates an error.
/// Referenced from the Anthropic API documentation [here](https://docs.anthropic.com/en/api/errors#error-shapes).
#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicApiError {
    /// Type of the response, for error responses this will be "error".
    #[serde(rename = "type")]
    pub typ: String,
    /// Error message.
    pub error: AnthropicApiErrorBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicApiErrorBody {
    /// Type of the error (e.g., "not_found_error").
    #[serde(rename = "type")]
    pub typ: String,

    /// Error message.
    pub message: String,
}
