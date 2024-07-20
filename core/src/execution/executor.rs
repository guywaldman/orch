use std::{
    cell::{OnceCell, RefCell},
    pin::Pin,
};

use orch_response::{ResponseOption, ResponseOptions, ResponseSchemaField};
use thiserror::Error;
use tokio_stream::Stream;

use crate::lm::{
    LanguageModel, LanguageModelError, TextCompleteOptions, TextCompleteStreamOptions,
};

use super::ResponseFormat;

// use super::{ExecutorOptionResponseParser, ExecutorResponsOption, ExecutorResponseFormat};

pub(crate) const DEFAULT_PREAMBLE: &str = "You are a helpful assistant";

#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("General error: {0}")]
    General(LanguageModelError),

    #[error("Parsing LM response failed: {0}")]
    Parsing(String),
}

trait Executor<'a, L>
where
    L: LanguageModel + 'a,
{
    /// Generates a text completion from the LLM (non-streaming).
    async fn text_complete(
        &self,
        prompt: &str,
    ) -> Result<ExecutorTextCompleteResponse<String>, ExecutorError> {
        text_complete(self.lm(), prompt, &self.system_prompt()).await
    }

    /// System prompt (instructions) for the model.
    fn system_prompt(&self) -> String {
        let cell = OnceCell::new();
        cell.get_or_init(|| {
            let response_options = self.response_options().unwrap_or_default();
            generate_system_prompt(
                self.format(),
                self.preamble().unwrap_or(DEFAULT_PREAMBLE),
                &response_options,
            )
        })
        .clone()
    }

    fn response_options(&self) -> Option<Vec<ResponseOption>> {
        None
    }

    fn format(&self) -> ResponseFormat;

    fn preamble(&self) -> Option<&str> {
        Some("You are a helpful assistant")
    }

    fn lm(&self) -> &'a L;
}

pub struct TextExecutor<'a, L>
where
    L: LanguageModel,
{
    pub(crate) lm: &'a L,
    pub(crate) preamble: Option<&'a str>,
}

impl<'a, L: LanguageModel> Executor<'a, L> for TextExecutor<'a, L> {
    fn format(&self) -> ResponseFormat {
        ResponseFormat::Text
    }

    fn lm(&self) -> &'a L {
        self.lm
    }

    fn preamble(&self) -> Option<&str> {
        self.preamble
    }
}

/// Trait for LLM execution.
/// This should be implemented for each LLM text generation use-case, where the system prompt
/// changes according to the trait implementations.
impl<'a, L> TextExecutor<'a, L>
where
    L: LanguageModel,
{
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

pub struct StructuredExecutor<'a, L, T>
where
    L: LanguageModel,
    T: serde::de::DeserializeOwned,
{
    pub(crate) lm: &'a L,
    pub(crate) preamble: Option<&'a str>,
    pub(crate) response_options: &'a dyn ResponseOptions<T>,
    pub(crate) format: ResponseFormat,
}

impl<'a, L: LanguageModel, T: serde::de::DeserializeOwned> Executor<'a, L>
    for StructuredExecutor<'a, L, T>
{
    fn format(&self) -> ResponseFormat {
        ResponseFormat::Json
    }

    fn response_options(&self) -> Option<Vec<ResponseOption>> {
        Some(self.response_options.options())
    }

    fn lm(&self) -> &'a L {
        self.lm
    }

    fn preamble(&self) -> Option<&str> {
        self.preamble
    }
}

/// Trait for LLM execution.
/// This should be implemented for each LLM text generation use-case, where the system prompt
/// changes according to the trait implementations.
impl<'a, L, T> StructuredExecutor<'a, L, T>
where
    L: LanguageModel,
    T: serde::de::DeserializeOwned,
{
    /// Generates a structured response from the LLM (non-streaming).
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
    ) -> Result<ExecutorTextCompleteResponse<T>, ExecutorError> {
        let text_result = self.text_complete(prompt).await?;
        let result = self
            .response_options
            .parse(&text_result.content)
            .map_err(|e| {
                ExecutorError::Parsing(format!(
                    "Error while parsing response: {e}\nResponse: {:?}",
                    text_result.content
                ))
            })?;
        // TODO: Add error correction and handling.
        Ok(ExecutorTextCompleteResponse {
            content: result,
            context: ExecutorContext {},
        })
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

pub async fn text_complete<L: LanguageModel>(
    lm: &L,
    prompt: &str,
    system_prompt: &str,
) -> Result<ExecutorTextCompleteResponse<String>, ExecutorError> {
    let options = TextCompleteOptions {
        ..Default::default()
    };
    let response = lm
        .text_complete(prompt, system_prompt, options)
        .await
        .map_err(ExecutorError::General)?;
    Ok(ExecutorTextCompleteResponse {
        content: response.text,
        context: ExecutorContext {},
    })
}

pub async fn generate_embedding<L: LanguageModel>(
    lm: &L,
    prompt: &str,
) -> Result<Vec<f32>, ExecutorError> {
    let response = lm
        .generate_embedding(prompt)
        .await
        .map_err(ExecutorError::General)?;
    Ok(response)
}

pub fn generate_system_prompt(
    format: ResponseFormat,
    preamble: &str,
    response_options: &[ResponseOption],
) -> String {
    match format {
        ResponseFormat::Text => {
            // In the case of text, the choices are ignored since the choice cannot
            // be represented in the text format.
            preamble.to_owned()
        }
        ResponseFormat::Json => {
            let all_types = response_options
                .iter()
                .map(|option| option.type_name.clone())
                .collect::<Vec<_>>();

            let response_options_text = response_options
                .iter()
                .map(|option| {
                    let mut schema_text = String::new();
                    let mut schema_example = "{".to_string();
                    let type_field = ResponseSchemaField {
                        // NOTE: This is assumed by [`orch_response_derive`] to be the discriminator field.
                        name: "response_type".to_string(),
                        description: format!(
                            "The type of the response (\"{}\" in this case)",
                            option.type_name
                        )
                        .to_string(),
                        typ: "string".to_string(),
                        example: all_types.first().unwrap().to_string(),
                    };

                    for (i, field) in option
                        .schema
                        .iter()
                        .chain(std::iter::once(&type_field))
                        .enumerate()
                    {
                        schema_text.push_str(&format!(
                            "  - `{}` of type {} (description: {})\n\n",
                            field.name, field.typ, field.description
                        ));
                        schema_example
                            .push_str(&format!("\"{}\": \"{}\"", field.name, field.example));

                        if i < option.schema.len() - 1 {
                            schema_example.push(',');
                        }
                    }
                    schema_example.push('}');

                    format!(
                        "SCENARIO: {}\nDESCRIPTION: {}\nSCHEMA:\n{}\nEXAMPLE RESPONSE: {}\n\n\n",
                        option.scenario, option.description, schema_text, schema_example
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");

            let system_prompt = format!(
                "{preamble}
                            You have {choices_len} choices to respond, in a JSON format:
                            {response_options_text}
                    ",
                preamble = preamble,
                choices_len = response_options.len(),
                response_options_text = response_options_text
            )
            .trim()
            .to_string();
            system_prompt
        }
    }
}
