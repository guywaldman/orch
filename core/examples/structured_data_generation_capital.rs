//! This example demonstrates how to use the `Executor` to generate a structured response from the LLM.
//! Run like so: `cargo run --example structured_data_generation_capital -- France`

#![allow(dead_code)]

use orch::execution::*;
use orch::response::*;

mod example_utils;
use example_utils::get_lm;

#[derive(Variants, serde::Deserialize)]
pub enum ResponseVariants {
    Answer(AnswerResponseVariant),
    Fail(FailResponseVariant),
}

#[derive(Variant, serde::Deserialize)]
#[variant(
    variant = "Answer",
    scenario = "You know the capital city of the country",
    description = "Capital city of the country"
)]
pub struct AnswerResponseVariant {
    #[schema(
        description = "Capital city of the received country",
        example = "London"
    )]
    pub capital: String,
}

#[derive(Variant, serde::Deserialize)]
#[variant(
    variant = "Fail",
    scenario = "You don't know the capital city of the country",
    description = "Reason why the capital city is not known"
)]
pub struct FailResponseVariant {
    #[schema(
        description = "Reason why the capital city is not known",
        example = "Country 'foobar' does not exist"
    )]
    pub reason: String,
}

#[tokio::main]
async fn main() {
    let (lm, _) = get_lm();

    let country = "France";

    let executor = StructuredExecutorBuilder::new()
        .with_lm(&*lm)
        .with_preamble("
            You are a geography expert who helps users understand the capital city of countries around the world.
            You will receive a country name, and you will need to provide the capital city of that country.
            ")
        .with_options(Box::new(variants!(ResponseVariants)))
        .try_build()
        .unwrap();
    let response = executor.execute(country).await.expect("Execution failed");

    match response.content {
        ResponseVariants::Answer(answer) => {
            println!("Capital city of {}: {}", country, answer.capital);
            assert_eq!(answer.capital, "Paris");
        }
        ResponseVariants::Fail(fail) => {
            println!("Model failed to generate a response: {}", fail.reason);
        }
    }
}
