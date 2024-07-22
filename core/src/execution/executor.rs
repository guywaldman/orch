use std::pin::Pin;

use thiserror::Error;
use tokio_stream::Stream;

use crate::{
    alignment::AlignmentError,
    lm::{LanguageModel, LanguageModelError, OllamaError, TextCompleteOptions},
};

#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("{0}")]
    General(LanguageModelError),

    #[error("{0}")]
    LanguageModelError(LanguageModelError),

    #[error("Error when calling Ollama API: {0}")]
    OllamaApi(String),

    #[error("Parsing LM response failed: {0}")]
    Parsing(String),

    #[error("Alignment error: {0}")]
    Alignment(AlignmentError),
}

impl From<LanguageModelError> for ExecutorError {
    fn from(val: LanguageModelError) -> Self {
        match val {
            LanguageModelError::Ollama(OllamaError::Api(e)) => ExecutorError::OllamaApi(e),
            e => ExecutorError::LanguageModelError(e),
        }
    }
}

pub(crate) trait Executor<'a> {
    /// Generates a text completion from the LLM (non-streaming).
    async fn text_complete(
        &self,
        prompt: &str,
    ) -> Result<ExecutorTextCompleteResponse<String>, ExecutorError> {
        text_complete(self.lm(), prompt, &self.system_prompt()).await
    }

    /// System prompt (instructions) for the model.
    fn system_prompt(&self) -> String;

    fn lm(&self) -> &'a dyn LanguageModel;
}

// TODO: Support context for completions (e.g., IDs of past conversations in Ollama).
pub struct ExecutorContext;

pub struct ExecutorTextCompleteResponse<T> {
    pub content: T,
    pub context: ExecutorContext,
}

pub struct ExecutorTextCompleteStreamResponse {
    pub stream: Pin<Box<dyn Stream<Item = Result<String, LanguageModelError>> + Send>>,
    pub context: ExecutorContext,
}

pub async fn text_complete<'a>(
    lm: &'a dyn LanguageModel,
    prompt: &str,
    system_prompt: &str,
) -> Result<ExecutorTextCompleteResponse<String>, ExecutorError> {
    let options = TextCompleteOptions {
        ..Default::default()
    };
    let response = lm
        .text_complete(prompt, system_prompt, options)
        .await
        .map_err(ExecutorError::from)?;
    Ok(ExecutorTextCompleteResponse {
        content: response.text,
        context: ExecutorContext {},
    })
}

pub(crate) async fn generate_embedding<'a>(
    lm: &'a dyn LanguageModel,
    prompt: &str,
) -> Result<Vec<f32>, ExecutorError> {
    let response = lm
        .generate_embedding(prompt)
        .await
        .map_err(ExecutorError::from)?;
    Ok(response)
}
