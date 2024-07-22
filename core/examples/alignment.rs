#![allow(dead_code)]

use orch::alignment::AlignmentStrategyBuilder;
use orch::execution::*;
use orch::lm::*;
use orch::response::*;

#[derive(Variants, serde::Deserialize)]
pub enum ResponseVariants {
    Answer(AnswerResponseVariant),
    Fail(FailResponseVariant),
}

#[derive(Variant, serde::Deserialize)]
#[variant(
    variant = "Answer",
    scenario = "You know the answer",
    description = "Result of the calculation"
)]
pub struct AnswerResponseVariant {
    #[schema(description = "Result of the calculation", example = "42")]
    pub result: String,
}

#[derive(Variant, serde::Deserialize)]
#[variant(
    variant = "Fail",
    scenario = "You don't know the answer",
    description = "Reason why the answer is not known"
)]
pub struct FailResponseVariant {
    #[schema(
        description = "Reason why the answer is not known",
        example = "The phrase is not a mathematical related expression"
    )]
    pub reason: String,
}

#[tokio::main]
async fn main() {
    // We use a large foundational model for the main task.
    let ollama_large = OllamaBuilder::new()
        .with_model(ollama_model::LLAMA3_8B.to_string())
        .try_build()
        .unwrap();

    // We use a smaller model for the correction.
    let ollama_corrector = OllamaBuilder::new()
        .with_model(ollama_model::LLAMA3_8B.to_string())
        .try_build()
        .unwrap();

    // We define an alignment strategy that uses the correction model.
    let alignment_strategy = AlignmentStrategyBuilder::new()
        .with_retries(2)
        .with_lm(Box::new(ollama_corrector))
        .try_build()
        .unwrap();

    let executor = StructuredExecutorBuilder::new()
	.with_lm(&ollama_large)
	.with_preamble("
		You are a mathematician who helps users understand the result of mathematical expressions.
		You will receive a mathematical expression, and you will need to provide the result of that expression.
	")
	.with_options(Box::new(variants!(ResponseVariants)))
    .with_alignment(alignment_strategy)
	.try_build()
	.unwrap();
    let response = executor.execute("2 + 2").await.expect("Execution failed");

    match response.content {
        ResponseVariants::Answer(answer) => {
            println!("Result: {}", answer.result);
        }
        ResponseVariants::Fail(fail) => {
            println!("Model failed to generate a response: {}", fail.reason);
        }
    }
}
