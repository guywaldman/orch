use thiserror::Error;

use crate::{Llm, LlmError, TextCompleteOptions};

pub struct Executor<'a, L: Llm> {
    llm: &'a L,
}

#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("LLM error: {0}")]
    Llm(LlmError),
}

impl<'a, L: Llm> Executor<'a, L> {
    pub fn new(llm: &'a L) -> Self {
        Self { llm }
    }

    pub async fn text_complete(
        &self,
        prompt: &str,
        system_prompt: &str,
    ) -> Result<ExecutorResponse, ExecutorError> {
        let options = TextCompleteOptions {
            ..Default::default()
        };
        let response = self
            .llm
            .text_complete(prompt, system_prompt, options)
            .await
            .map_err(ExecutorError::Llm)?;
        Ok(ExecutorResponse {
            text: response.text,
            context: ExecutorContext {},
        })
    }
}

// TODO: Support context for completions (e.g., IDs of past conversations in Ollama).
pub struct ExecutorContext;

pub struct ExecutorResponse {
    pub text: String,
    pub context: ExecutorContext,
}
