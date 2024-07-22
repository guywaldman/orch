use std::cell::OnceCell;

use orch_response::{OrchResponseVariants, ResponseSchemaField};

use crate::{alignment::AlignmentStrategy, lm::LanguageModel};

use super::{
    generate_embedding, Executor, ExecutorBuilderError, ExecutorContext, ExecutorError,
    ExecutorTextCompleteResponse, DEFAULT_PREAMBLE,
};

pub struct StructuredExecutor<'a, T> {
    pub(crate) lm: &'a dyn LanguageModel,
    pub(crate) preamble: Option<&'a str>,
    pub(crate) variants: Box<dyn OrchResponseVariants<T>>,
    pub(crate) alignment_strategy: Option<AlignmentStrategy>,
}

impl<'a, T> Executor<'a> for StructuredExecutor<'a, T> {
    fn lm(&self) -> &'a dyn LanguageModel {
        self.lm
    }

    fn system_prompt(&self) -> String {
        let cell = OnceCell::new();

        cell.get_or_init(|| {
            let response_options = self.variants.variants();
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

            // Add an optional extra preamble supplied by the user.
            let preamble = self.preamble.map(|pa| format!("Additional information: {}", pa)).unwrap_or("".to_owned());

            let system_prompt = format!(
                "
								You will receive a prompt from a user, and will need to response with a JSON object that represents the response.
								Response *only* with the JSON object, and nothing else. No additional preamble or explanations. Only work with the responses you can reply with.
								{preamble}

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
        })
        .clone()
    }
}

/// Trait for LLM execution.
/// This should be implemented for each LLM text generation use-case, where the system prompt
/// changes according to the trait implementations.
impl<'a, T> StructuredExecutor<'a, T> {
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
        let mut model_response = self.text_complete(prompt).await?.content;
        if let Some(alignment_strategy) = &self.alignment_strategy {
            model_response = alignment_strategy
                .align(
                    self.lm,
                    self.preamble.unwrap_or(DEFAULT_PREAMBLE),
                    prompt,
                    &model_response,
                )
                .await
                .map_err(ExecutorError::Alignment)?;
        }
        let result = self.variants.parse(&model_response).map_err(|e| {
            ExecutorError::Parsing(format!(
                "Error while parsing response: {e}\nResponse: {:?}",
                model_response
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

#[derive(Default)]
pub struct StructuredExecutorBuilder<'a, T> {
    lm: Option<&'a dyn LanguageModel>,
    preamble: Option<&'a str>,
    variants: Option<Box<dyn OrchResponseVariants<T>>>,
    alignment_strategy: Option<AlignmentStrategy>,
}

impl<'a, T> StructuredExecutorBuilder<'a, T> {
    pub fn new() -> Self {
        Self {
            lm: None,
            preamble: None,
            variants: None,
            alignment_strategy: None,
        }
    }

    pub fn with_lm(mut self, lm: &'a dyn LanguageModel) -> Self {
        self.lm = Some(lm);
        self
    }

    pub fn with_options(mut self, options: Box<dyn OrchResponseVariants<T>>) -> Self {
        self.variants = Some(options);
        self
    }

    pub fn with_preamble(mut self, preamble: &'a str) -> Self {
        self.preamble = Some(preamble);
        self
    }

    pub fn with_alignment(mut self, strategy: AlignmentStrategy) -> Self {
        self.alignment_strategy = Some(strategy);
        self
    }

    pub fn try_build(self) -> Result<StructuredExecutor<'a, T>, ExecutorBuilderError> {
        let Some(lm) = self.lm else {
            return Err(ExecutorBuilderError::ConfigurationNotSet(
                "Language model".to_string(),
            ));
        };
        let Some(response_options) = self.variants else {
            return Err(ExecutorBuilderError::ConfigurationNotSet(
                "Response options".to_string(),
            ));
        };
        Ok(StructuredExecutor {
            lm,
            preamble: self.preamble,
            variants: response_options,
            alignment_strategy: self.alignment_strategy,
        })
    }
}
