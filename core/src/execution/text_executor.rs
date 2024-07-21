use crate::lm::{LanguageModel, TextCompleteStreamOptions};

use super::{
    generate_embedding, Executor, ExecutorBuilderError, ExecutorContext, ExecutorError,
    ExecutorTextCompleteResponse, ExecutorTextCompleteStreamResponse, ResponseFormat,
};

pub const DEFAULT_PREAMBLE: &str = "You are a helpful assistant";

pub struct TextExecutor<'a> {
    pub(crate) lm: &'a dyn LanguageModel,
    pub(crate) preamble: Option<&'a str>,
}

impl<'a> Executor<'a> for TextExecutor<'a> {
    fn format(&self) -> ResponseFormat {
        ResponseFormat::Text
    }

    fn lm(&self) -> &'a dyn LanguageModel {
        self.lm
    }

    fn system_prompt(&self) -> String {
        self.preamble.unwrap_or(&DEFAULT_PREAMBLE).to_owned()
    }
}

/// Trait for LLM execution.
/// This should be implemented for each LLM text generation use-case, where the system prompt
/// changes according to the trait implementations.
impl<'a> TextExecutor<'a> {
    /// Generates a streaming response from the LLM.
    ///
    /// # Arguments
    /// * `prompt` - The prompt to generate a response for.
    /// * `system_prompt` - The system prompt to use for the generation.
    ///
    /// # Returns
    /// A [Result] containing the response from the LLM or an error if there was a problem.
    pub async fn execute_stream(
        &'a self,
        prompt: &'a str,
    ) -> Result<ExecutorTextCompleteStreamResponse, ExecutorError> {
        let options = TextCompleteStreamOptions {
            ..Default::default()
        };
        let system_prompt = self.system_prompt();
        let response = self
            .lm
            .text_complete_stream(prompt, &system_prompt, options)
            .await
            .map_err(ExecutorError::General)?;
        Ok(ExecutorTextCompleteStreamResponse {
            stream: response.stream,
            context: ExecutorContext {},
        })
    }

    /// Generates a response from the LLM (non-streaming).
    ///
    /// # Arguments
    /// * `prompt` - The prompt to generate a response for.
    /// * `system_prompt` - The system prompt to use for the generation.
    ///
    /// # Returns
    /// A [Result] containing the response from the LLM or an error if there was a problem.
    pub async fn execute(
        &'a self,
        prompt: &'a str,
    ) -> Result<ExecutorTextCompleteResponse<String>, ExecutorError> {
        self.text_complete(prompt).await
    }

    /// Generates an embedding from the LLM.
    ///
    /// # Arguments
    /// * `prompt` - The item to generate an embedding for.
    ///
    /// # Returns
    ///
    /// A [Result] containing the embedding or an error if there was a problem.
    pub async fn generate_embedding(&'a self, prompt: &'a str) -> Result<Vec<f32>, ExecutorError> {
        generate_embedding(self.lm, prompt).await
    }
}

#[derive(Default)]
pub struct TextExecutorBuilder<'a> {
    lm: Option<&'a dyn LanguageModel>,
    preamble: Option<&'a str>,
}

impl<'a> TextExecutorBuilder<'a> {
    pub fn new() -> Self {
        Self {
            lm: None,
            preamble: None,
        }
    }

    pub fn with_lm(mut self, lm: &'a dyn LanguageModel) -> Self {
        self.lm = Some(lm);
        self
    }

    pub fn with_preamble(mut self, preamble: &'a str) -> Self {
        self.preamble = Some(preamble);
        self
    }

    pub fn try_build(self) -> Result<TextExecutor<'a>, ExecutorBuilderError> {
        let Some(lm) = self.lm else {
            return Err(ExecutorBuilderError::ConfigurationNotSet(
                "Language model".to_string(),
            ));
        };
        Ok(TextExecutor {
            lm,
            preamble: self.preamble,
        })
    }
}
