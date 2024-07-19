use std::pin::Pin;

use thiserror::Error;
use tokio_stream::Stream;

use crate::{Llm, LlmError, TextCompleteOptions, TextCompleteStreamOptions};

pub struct Executor<'a, L: Llm> {
    llm: &'a L,
}

#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("LLM error: {0}")]
    Llm(LlmError),
}

impl<'a, L: Llm> Executor<'a, L> {
    /// Creates a new `Executor` instance.
    ///
    /// # Arguments
    /// * `llm` - The LLM to use for the execution.
    pub fn new(llm: &'a L) -> Self {
        Self { llm }
    }

    /// Generates a response from the LLM (non-streaming).
    ///
    /// # Arguments
    /// * `prompt` - The prompt to generate a response for.
    /// * `system_prompt` - The system prompt to use for the generation.
    ///
    /// # Returns
    /// A [Result] containing the response from the LLM or an error if there was a problem.
    pub async fn text_complete(
        &self,
        prompt: &str,
        system_prompt: &str,
    ) -> Result<ExecutorTextCompleteResponse, ExecutorError> {
        let options = TextCompleteOptions {
            ..Default::default()
        };
        let response = self
            .llm
            .text_complete(prompt, system_prompt, options)
            .await
            .map_err(ExecutorError::Llm)?;
        Ok(ExecutorTextCompleteResponse {
            text: response.text,
            context: ExecutorContext {},
        })
    }

    /// Generates a streaming response from the LLM.
    ///
    /// # Arguments
    /// * `prompt` - The prompt to generate a response for.
    /// * `system_prompt` - The system prompt to use for the generation.
    ///
    /// # Returns
    /// A [Result] containing the response from the LLM or an error if there was a problem.
    pub async fn text_complete_stream(
        &self,
        prompt: &str,
        system_prompt: &str,
    ) -> Result<ExecutorTextCompleteStreamResponse, ExecutorError> {
        let options = TextCompleteStreamOptions {
            ..Default::default()
        };
        let response = self
            .llm
            .text_complete_stream(prompt, system_prompt, options)
            .map_err(ExecutorError::Llm)?;
        Ok(ExecutorTextCompleteStreamResponse {
            stream: response.stream,
            context: ExecutorContext {},
        })
    }
}

// TODO: Support context for completions (e.g., IDs of past conversations in Ollama).
pub struct ExecutorContext;

pub struct ExecutorTextCompleteResponse {
    pub text: String,
    pub context: ExecutorContext,
}

pub struct ExecutorTextCompleteStreamResponse {
    pub stream: Pin<Box<dyn Stream<Item = Result<String, LlmError>> + Send>>,
    pub context: ExecutorContext,
}
