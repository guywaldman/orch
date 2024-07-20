//! This example demonstrates how to use the `Executor` to generate a response from the LLM.
//! We construct an `Ollama` instance and use it to generate a response.

use orch::{
    execution::{StructuredExecutorBuilder, TextExecutorBuilder},
    lm::{LanguageModelBuilder, LanguageModelProvider, OllamaBuilder, OpenAiBuilder},
};

#[tokio::main]
async fn main() {
    // ! Change this to use a different provider.
    let provider = LanguageModelProvider::Ollama;

    let prompt = "What is 2+2?";
    let system_prompt = "You are a helpful assistant";

    println!("Prompt: {prompt}");
    println!("System prompt: {system_prompt}");
    println!("---");

    let lm = match provider {
        LanguageModelProvider::Ollama => OllamaBuilder::new().try_build().unwrap(),
        LanguageModelProvider::OpenAi => OpenAiBuilder::new().try_build().unwrap(),
    };
    let executor = TextExecutorBuilder::new().with_lm(&lm).try_build().unwrap();
    let response = executor.execute(prompt).await.expect("Execution failed");

    println!("Response:");
    println!("{}", response.content);
}
