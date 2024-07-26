use serde::{Deserialize, Serialize};

/// Request for generating a response from the Anthropic API.
/// Referenced from the Anthropic API documentation [here](https://docs.anthropic.com/en/api/complete).
#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicCompleteApiRequest {
    /// The model that will complete your prompt.
    /// See [`anthropic_model`] for a list of built-in model IDs for convenience.
    ///
    /// See [models](https://docs.anthropic.com/en/docs/about-claude/models) for a complete list of models supported by Anthropic.
    pub model: String,

    /// The prompt that you want Claude to complete.
    /// For proper response generation you will need to format your prompt using alternating \n\nHuman: and \n\nAssistant: conversational turns. For example:
    /// "\n\nHuman: {userQuestion}\n\nAssistant:"
    /// See [prompt validation](https://docs.anthropic.com/en/api/prompt-validation) and the Anthropic [prompt design guide](https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering/overview)
    /// for more information and guidance.
    pub prompt: String,

    /// The maximum number of tokens to generate before stopping.
    ///
    /// Note that the Anthropic models may stop before reaching this maximum. This parameter only specifies the absolute maximum number of tokens to generate.
    pub max_tokens: Option<usize>,

    /// Sequences that will cause the model to stop generating.
    /// The Anthropic models stop on "\n\nHuman:", and may include additional built-in stop sequences in the future.
    /// By providing the stop_sequences parameter, you may include additional strings that will cause the model to stop generating.
    pub stop_sequences: Option<Vec<String>>,

    /// Amount of randomness injected into the response.
    ///
    /// Defaults to 1.0. Ranges from 0.0 to 1.0. Use temperature closer to 0.0 for analytical / multiple choice, and closer to 1.0 for creative and generative tasks.
    ///
    // Note that even with temperature of 0.0, the results will not be fully deterministic.
    pub temperature: Option<f32>,

    /// Only sample from the top K options for each subsequent token.
    ///
    /// Used to remove "long tail" low probability responses. Learn more technical details [here](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277).
    ///
    /// Recommended for advanced use cases only. You usually only need to use temperature.
    pub top_k: Option<usize>,
}

/// Response from the Anthropic API for generating a response.
/// Referenced from the Anthropic API documentation [here](https://docs.anthropic.com/en/api/complete).
#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicCompleteApiResponse {
    /// Object type. For text completion, this will be "completion".
    #[serde(rename = "type")]
    pub typ: String,

    /// The resulting completion up to and excluding the stop sequences.
    pub completion: String,

    /// The reason that the model stopped generating tokens.
    ///
    /// This may be one the following values:
    /// - "stop_sequence": Reached a stop sequence — either provided by you via the stop_sequences parameter, or a stop sequence built into the model
    /// - "max_tokens": Exceeded `max_tokens_to_sample` or the model's maximum
    pub stop_reason: Option<String>,
}

/// Convenience constants for the Anthropic models.
pub mod anthropic_model {
    pub const CLAUDE_3_5_SONNET: &str = "claude-3-5-sonnet-20240620";
    pub const CLAUDE_3_OPUS: &str = "claude-3-opus-20240229";
    pub const CLAUDE_3_SONNET: &str = "claude-3-sonnet-20240229";
    pub const CLAUDE_3_HAIKU: &str = "claude-3-haiku-20240307";
}
