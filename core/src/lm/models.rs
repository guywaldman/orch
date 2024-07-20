#![allow(dead_code)]

use std::pin::Pin;

use dyn_clone::DynClone;
use tokio_stream::Stream;

use super::{error::LanguageModelError, LanguageModelProvider};

/// A trait for language model providers which implements text completion, embeddings, etc.
///
/// > `DynClone` is used so that there can be dynamic dispatch of the `Llm` trait,
/// > especially needed for [magic-cli](https://github.com/guywaldman/magic-cli).
pub trait LanguageModel: DynClone {
    /// Generates a response from the LLM.
    ///
    /// # Arguments
    /// * `prompt` - The prompt to generate a response for.
    /// * `system_prompt` - The system prompt to use for the generation.
    /// * `options` - The options for the generation.
    ///
    /// # Returns
    /// A [Result] containing the response from the LLM or an error if there was a problem.
    ///
    async fn text_complete(
        &self,
        prompt: &str,
        system_prompt: &str,
        options: TextCompleteOptions,
    ) -> Result<TextCompleteResponse, LanguageModelError>;

    /// Generates a streaming response from the LLM.
    ///
    /// # Arguments
    /// * `prompt` - The prompt to generate a response for.
    /// * `system_prompt` - The system prompt to use for the generation.
    /// * `options` - The options for the generation.
    ///
    /// # Returns
    /// A [Result] containing the response from the LLM or an error if there was a problem.
    ///
    async fn text_complete_stream(
        &self,
        prompt: &str,
        system_prompt: &str,
        options: TextCompleteStreamOptions,
    ) -> Result<TextCompleteStreamResponse, LanguageModelError>;

    /// Generates an embedding from the LLM.
    ///
    /// # Arguments
    /// * `prompt` - The item to generate an embedding for.
    ///
    /// # Returns
    ///
    /// A [Result] containing the embedding or an error if there was a problem.
    async fn generate_embedding(&self, prompt: &str) -> Result<Vec<f32>, LanguageModelError>;

    /// Returns the provider of the LLM.
    fn provider(&self) -> LanguageModelProvider;

    /// Returns the name of the model used for text completions.
    fn text_completion_model_name(&self) -> String;

    /// Returns the name of the model used for embeddings.
    fn embedding_model_name(&self) -> String;
}

#[derive(Debug, Clone, Default)]
pub struct TextCompleteOptions {
    /// An encoding of the conversation used in this response, this can be sent in the next request to keep a conversational memory.
    /// This should be as returned from the previous response.
    pub context: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Default)]
pub struct TextCompleteStreamOptions {
    pub context: Option<Vec<i64>>,
}

#[derive(Debug, Clone)]
pub struct TextCompleteResponse {
    pub text: String,
    // TODO: This is specific to Ollama, context looks differently for other LLM providers.
    pub context: Option<Vec<i64>>,
}

pub struct TextCompleteStreamResponse {
    pub stream: Pin<Box<dyn Stream<Item = Result<String, LanguageModelError>> + Send>>,
    // TODO: Handle context with streaming response.
    // pub context: Vec<i64>,
}
