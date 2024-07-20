//! This example demonstrates how to use the `Executor` to generate a response from the LLM.
//! We construct an `Ollama` instance and use it to generate a response.

use orch::{
    execution::{StructuredExecutorBuilder, TextExecutorBuilder},
    lm::OllamaBuilder,
};

#[tokio::main]
async fn main() {
    let prompt = "What is 2+2?";
    let system_prompt = "You are a helpful assistant";

    println!("Prompt: {prompt}");
    println!("System prompt: {system_prompt}");
    println!("---");

    let ollama = OllamaBuilder::new().build();
    let executor = TextExecutorBuilder::new()
        .with_lm(&ollama)
        .try_build()
        .unwrap();
    let response = executor.execute(prompt).await.expect("Execution failed");

    println!("Response:");
    println!("{}", response.content);
}
