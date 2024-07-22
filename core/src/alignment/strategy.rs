use async_recursion::async_recursion;
use orch_response_derive::{variants, Variant, Variants};
use thiserror::Error;

use crate::{
    execution::{ExecutorError, StructuredExecutor, StructuredExecutorBuilder},
    lm::{LanguageModel, LanguageModelError, TextCompleteOptions},
};

#[derive(Debug, Error)]
pub enum AlignmentError {
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Language model error: {0}")]
    LanguageModelError(#[from] LanguageModelError),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Max retries exceeded ({0} retries)")]
    MaxRetriesExceeded(usize),
}

pub struct AlignmentStrategy {
    pub(crate) lm: Box<dyn LanguageModel>,
    pub(crate) retries: usize,
}

#[derive(Variants, Clone, serde::Deserialize)]
pub enum AlignmentResponse {
    ResponseCorrection(ResponseCorrectionResponseVariant),
    SchemaCorrection(SchemaCorrectionResponseVariant),
    NoCorrection(NoCorrectionResponseVariant),
    Fail(FailResponseVariant),
}

#[derive(Variant, Clone, serde::Deserialize)]
#[variant(
    variant = "ResponseCorrection",
    scenario = "The response format is correct, but the response content itself is incorrect",
    description = "A correction and a reason why it is needed"
)]
pub struct ResponseCorrectionResponseVariant {
    #[schema(
        description = "Correction of the phrase",
        example = "{ \"capital\": \"Paris\" }"
    )]
    pub correction: String,

    #[schema(
        description = "Short reason why a correction is needed",
        example = "The capital of France is not London as the original model returned, but Paris"
    )]
    pub reason: String,
}

#[derive(Variant, Clone, serde::Deserialize)]
#[variant(
    variant = "SchemaCorrection",
    scenario = "The schema of the response is incorrect",
    description = "Explanation of why the schema is incorrect"
)]
pub struct SchemaCorrectionResponseVariant {
    #[schema(
        description = "Correction of the schema, in natural language",
        example = "\"'capital' should be a string, not a number'\" or \"The 'capital' field has a typo and starts with an uppercase letter\""
    )]
    pub correction: String,

    #[schema(
        description = "Short reason why a correction is needed",
        example = "The 'capital' field is a number, not a string"
    )]
    pub reason: String,
}

#[derive(Variant, Clone, serde::Deserialize)]
#[variant(
    variant = "NoCorrection",
    scenario = "No correction needed, the original response satisfies the expected output",
    description = "Short reason why a correction is not needed"
)]
pub struct NoCorrectionResponseVariant {
    #[schema(
        description = "Short reason why a correction is not needed",
        example = "The user asked for the capital city of France, and the answer is indeed Paris"
    )]
    pub reason: String,
}

#[derive(Variant, Clone, serde::Deserialize)]
#[variant(
    variant = "Fail",
    scenario = "You don't know how to verify whether the answer is correct or not. You should only go for this response in extreme cases",
    description = "Reason why you failed to determine whether the answer is correct or not"
)]
pub struct FailResponseVariant {
    #[schema(
        description = "Reason why you failed to determine whether the answer is correct or not",
        example = "The question is extremely vague and the model returned something completely unrelated"
    )]
    pub reason: String,
}

impl AlignmentStrategy {
    const PREAMBLE: &'static str = "
    Your purpose is to receive a response from a language model and make sure (and correct otherwise) whether the response is expected or not.  
    Being \"expected\" means that the response is correct and matches the expected output.

    You should *not* return the response in the schema of the original message, but instead of the schema that you are requested to provide
    (the one with the response types 'ResponseCorrection', 'SchemaCorrection' and 'NoCorrection').
    ";

    /// Aligns the response of the language model.
    /// Tries at least once, and continues according to the [`AlignmentStrategy`]
    /// (e.g., number of retries).
    pub async fn align<'a>(
        &self,
        base_lm: &'a dyn LanguageModel,
        original_preamble: &str,
        original_prompt: &str,
        original_response: &str,
    ) -> Result<String, AlignmentError> {
        let mut iterated_response = original_response.to_owned();
        let mut retry_count = 0;
        let mut prev_alignment_response = None;

        loop {
            let response = self
                .request_correction(
                    original_preamble,
                    original_prompt,
                    &iterated_response,
                    &prev_alignment_response,
                )
                .await?;

            let Some(response) = response else {
                // The response may be `None` if the correction deemed that the previous response should be used.
                continue;
            };

            match &response {
                AlignmentResponse::NoCorrection(_) => {
                    // Found no correction, can return the original response.
                    return Ok(iterated_response.to_owned());
                }
                response => {
                    retry_count += 1;

                    if retry_count >= self.retries {
                        return Err(AlignmentError::MaxRetriesExceeded(retry_count));
                    }

                    if let AlignmentResponse::Fail(_) = response {
                        // Failed - simply try again.
                        continue;
                    }

                    let correction = match response {
                        AlignmentResponse::ResponseCorrection(response_correction) => {
                            response_correction.correction.clone()
                        }
                        AlignmentResponse::SchemaCorrection(schema_correction) => {
                            schema_correction.correction.clone()
                        }
                        _ => unreachable!(),
                    };

                    let correction_prompt = format!("
                        {original_preamble}

                        NOTE:
                        You have previously answered this with the following response and was incorrect. Here is the response and the correction, please make sure not to repeat the same mistake:
                        ORIGINAL RESPONSE: {original_response}
                        CORRECTION: {correction}
                        ");
                    let new_base_model_response = base_lm
                        .text_complete(
                            original_prompt,
                            &correction_prompt,
                            TextCompleteOptions::default(),
                        )
                        .await
                        .map_err(AlignmentError::LanguageModelError)?;

                    prev_alignment_response = Some(response.clone());
                    iterated_response = new_base_model_response.text;
                }
            }
        }
    }

    #[async_recursion]
    async fn request_correction(
        &self,
        original_preamble: &str,
        original_prompt: &str,
        original_response: &str,
        prev_alignment_response: &Option<AlignmentResponse>,
    ) -> Result<Option<AlignmentResponse>, AlignmentError> {
        let mut preamble = format!(
            "
            {base_preamble}

            The model received the original instructions:
            {original_preamble}

            And the original prompt:
            {original_prompt}

            And the original response:
            {original_response}

            REMEMBER: Return a response in the schema you are requested (the one with the response types 'ResponseCorrection', 'SchemaCorrection' and 'NoCorrection').
    ",
            base_preamble = Self::PREAMBLE,
        );
        // If `alignment_response` is `None`, then this was the first attempt and no additional preamble is needed.
        if let Some(prev_alignment_response) = prev_alignment_response {
            // TODO: Add context of more tries?
            preamble.push_str(&format!(
                "
                IMPORTANT CONTEXT:
                Before receiving the previous correction, the model has already responded with the following:

                {}

                And received the following corrections:
                ",
                original_response
            ));

            match prev_alignment_response {
                AlignmentResponse::ResponseCorrection(response_correction) => {
                    preamble.push_str(&format!(
                        "CORRECTION: The response content was incorrect, this is the correction: {}",
                        response_correction.correction,
                    ));
                }
                AlignmentResponse::SchemaCorrection(schema_correction) => {
                    preamble.push_str(&format!(
                        "CORRECTION: The response schema was incorrect for the following reason: {}
                        This is the correction: {}
                        ",
                        schema_correction.correction, schema_correction.reason
                    ));
                }
                _ => {
                    // No error (this is unexpected) - return the original response.
                    return Err(AlignmentError::InternalError(
                        "Requested correction with no relevant correction response".to_owned(),
                    ));
                }
            }
        }

        let executor: StructuredExecutor<AlignmentResponse> = StructuredExecutorBuilder::new()
            .with_lm(&*self.lm)
            .with_preamble(&preamble)
            .with_options(Box::new(variants!(AlignmentResponse)))
            .try_build()
            .unwrap();
        let response = {
            let correction_response = executor.execute(original_prompt).await;

            match correction_response {
                Ok(response) => Some(response.content),
                Err(ExecutorError::Parsing(_)) => {
                    // The model failed to parse the response, so we return the original response.
                    return Ok(None);
                }
                Err(e) => return Err(AlignmentError::ExecutionFailed(e.to_string())),
            }
        };

        Ok(response)
    }
}
