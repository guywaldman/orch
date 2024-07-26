//! This example demonstrates how to use the `Executor` to generate a streaming response from the LLM.
//! Run like so: `cargo run --example text_generation_stream`

use orch::execution::*;
use tokio_stream::StreamExt;

mod example_utils;
use example_utils::get_lm;

#[tokio::main]
async fn main() {
    let lm = get_lm(
        std::env::args()
            .nth(1)
            .unwrap_or("ollama".to_owned())
            .as_str(),
    );

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
