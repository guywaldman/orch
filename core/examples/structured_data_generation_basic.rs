#![allow(dead_code)]
//! This example demonstrates how to use the `Executor` to generate a structured response from the LLM.

use orch::execution::*;
use orch::lm::*;
use orch::response::*;

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
    #[schema(
        field = "country",
        description = "Country of the received capital city",
        example = "United Kingdom"
    )]
    Answer { capital: String, country: String },
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
    // ! Change this to use a different provider.
    let provider = LanguageModelProvider::Ollama;

    let prompt = "What is the capital of France?";

    println!("Prompt: {prompt}");
    println!("---");

    // Use a different language model, per the `provider` variable (feel free to change it).
    let open_ai_api_key = {
        if provider == LanguageModelProvider::OpenAi {
            std::env::var("OPENAI_API_KEY")
                .unwrap_or_else(|_| panic!("OPENAI_API_KEY environment variable not set"))
        } else {
            String::new()
        }
    };
    let lm: Box<dyn LanguageModel> = match provider {
        LanguageModelProvider::Ollama => Box::new(OllamaBuilder::new().try_build().unwrap()),
        LanguageModelProvider::OpenAi => Box::new(
            OpenAiBuilder::new()
                .with_api_key(&open_ai_api_key)
                .try_build()
                .unwrap(),
        ),
    };

    let executor = StructuredExecutorBuilder::new()
        .with_lm(&*lm)
        .with_preamble("You are a geography expert who helps users understand the capital city of countries around the world.")
        .with_options(&options!(CapitalCityExecutorResponseOptions))
        .try_build()
        .unwrap();
    let response = executor.execute(prompt).await.expect("Execution failed");

    println!("Response:");
    println!("{:?}", response.content);
}
