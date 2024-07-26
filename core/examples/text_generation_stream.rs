//! This example demonstrates how to use the `Executor` to generate a streaming response from the LLM.
//! Run like so: `cargo run --example text_generation_stream`

use orch::execution::*;
use orch::lm::*;
use tokio_stream::StreamExt;

mod example_utils;
use example_utils::get_lm;

// ! Change this to use a different provider.
pub const PROVIDER: LanguageModelProvider = LanguageModelProvider::Ollama;

#[tokio::main]
async fn main() {
    let lm = get_lm(PROVIDER);

    let prompt = "What is 2+2?";

    println!("Prompt: {prompt}");
    println!("---");

    let executor = TextExecutorBuilder::new()
        .with_lm(&*lm)
        .try_build()
        .unwrap();
    let mut response = executor
        .execute_stream(prompt)
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
