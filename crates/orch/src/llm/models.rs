#![allow(dead_code)]

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;

use super::error::LlmError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LlmProvider {
    #[serde(rename = "ollama")]
    Ollama,
    #[serde(rename = "openai")]
    OpenAi,
}

/// A trait for LLM providers which implements text completion, embeddings, etc.
///
/// > `DynClone` is used so that there can be dynamic dispatch of the `Llm` trait,
/// > especially needed for [magic-cli](https://github.com/guywaldman/magic-cli).
pub trait Llm: DynClone {
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
    fn text_complete(
        &self,
        prompt: &str,
        system_prompt: &str,
        options: TextCompleteOptions,
    ) -> Result<TextCompleteResponse, LlmError>;

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
    fn text_complete_stream(
        &self,
        prompt: &str,
        system_prompt: &str,
        options: TextCompleteStreamOptions,
    ) -> Result<TextCompleteStreamResponse, LlmError>;

    /// Generates an embedding from the LLM.
    ///
    /// # Arguments
    /// * `prompt` - The item to generate an embedding for.
    ///
    /// # Returns
    ///
    /// A [Result] containing the embedding or an error if there was a problem.
    fn generate_embedding(&self, prompt: &str) -> Result<Vec<f32>, LlmError>;

    /// Returns the provider of the LLM.
    fn provider(&self) -> LlmProvider;

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
    pub context: Vec<i64>,
}

pub struct TextCompleteStreamResponse {
    pub stream: Pin<Box<dyn Stream<Item = String> + Send>>,
    // TODO: Handle context with streaming response.
    // pub context: Vec<i64>,
}

#[derive(Debug)]
pub(crate) struct SystemPromptResponseOption {
    pub scenario: String,
    pub type_name: String,
    pub response: String,
    pub schema: Vec<SystemPromptCommandSchemaField>,
}

#[derive(Debug)]
pub(crate) struct SystemPromptCommandSchemaField {
    pub name: String,
    pub description: String,
    pub typ: String,
    pub example: String,
}
