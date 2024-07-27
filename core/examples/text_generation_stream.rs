//! This example demonstrates how to use the `Executor` to generate a streaming response from the LLM.
//! Run like so: `cargo run --example text_generation_stream`

use orch::{execution::*, lm::LanguageModelProvider};
use tokio_stream::StreamExt;

mod example_utils;
use example_utils::get_lm;

#[tokio::main]
async fn main() {
    let (lm, provider) = get_lm();

    if provider == LanguageModelProvider::Anthropic {
        println!("Streaming is not currently supported for Anthropic. Skipping example.");
        return;
    }

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

    let mut response_text = String::new();
    println!("Response:");
    while let Some(chunk) = response.stream.next().await {
        match chunk {
            Ok(chunk) => {
                print!("{chunk}");
                response_text.push_str(&chunk);
            }
            Err(e) => {
                println!("Error: {e}");
                break;
            }
        }
    }
    println!();

    assert!(!response_text.is_empty());
}
