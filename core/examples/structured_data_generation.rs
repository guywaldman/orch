//! This example demonstrates how to use the `Executor` to generate a structured response from the LLM.

use orch::{
    execution::{ResponseFormat, StructuredExecutorBuilder},
    lm::OllamaBuilder,
};
use orch_response_derive::{options, OrchResponseOptions};

#[derive(OrchResponseOptions)]
pub enum CapitalCityExecutorResponseOptions {
    #[response(
        scenario = "You know the capital city of the country",
        description = "Capital city of the country"
    )]
    #[schema(
        field = "capital",
        description = "Capital city of the received country",
        example = "London"
    )]
    Answer { capital: String },
    #[response(
        scenario = "You don't know the capital city of the country",
        description = "Reason why the capital city is not known"
    )]
    #[schema(
        field = "reason",
        description = "Reason why the capital city is not known",
        example = "Country 'foobar' does not exist"
    )]
    Fail { reason: String },
}

#[tokio::main]
async fn main() {
    let prompt = "What is the capital of Fooland?";
    let system_prompt = "You are a helpful assistant";

    println!("Prompt: {prompt}");
    println!("System prompt: {system_prompt}");
    println!("---");

    let ollama = OllamaBuilder::new().build();
    let executor = StructuredExecutorBuilder::new()
        .with_lm(&ollama)
        .with_preamble("You are a geography expert who helps users understand the capital city of countries around the world.")
        .with_options(&options!(CapitalCityExecutorResponseOptions))
        .try_build()
        .unwrap();
    let response = executor.execute(prompt).await.expect("Execution failed");

    println!("Response:");
    println!("{:?}", response.content);
}
