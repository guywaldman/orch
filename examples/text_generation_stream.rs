//! This example demonstrates how to use the `Executor` to generate a streaming response from the LLM.
//! We construct an `Ollama` instance and use it to generate a streaming response.

use orch::{Executor, OllamaBuilder};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let prompt = "What is 2+2?";
    let system_prompt = "You are a helpful assistant";

    println!("Prompt: {prompt}");
    println!("System prompt: {system_prompt}");
    println!("---");

    let ollama = OllamaBuilder::new().build();
    let executor = Executor::new(&ollama);
    let mut response = executor
        .text_complete_stream(prompt, system_prompt)
        .await
        .expect("Execution failed");

    println!("Response:");
    while let Some(chunk) = response.stream.next().await {
        match chunk {
            Ok(chunk) => print!("{chunk}"),
            Err(e) => {
                println!("Error: {e}");
                break;
            }
        }
    }
    println!();
}
