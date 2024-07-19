//! This example demonstrates how to use the `Executor` to generate a response from the LLM.
//! We construct an `Ollama` instance and use it to generate a response.

use orch::{Executor, OllamaBuilder};

#[tokio::main]
async fn main() {
    let prompt = "What is 2+2?";
    let system_prompt = "You are a helpful assistant";

    println!("Prompt: {prompt}");
    println!("System prompt: {system_prompt}");
    println!("---");

    let ollama = OllamaBuilder::new().build();
    let executor = Executor::new(&ollama);
    let response = executor
        .text_complete(prompt, system_prompt)
        .await
        .expect("Execution failed");

    println!("Response:");
    println!("{}", response.text);
}
