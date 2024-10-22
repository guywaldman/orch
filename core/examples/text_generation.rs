//! This example demonstrates how to use the `Executor` to generate a response from the LLM.
//! Run like so: `cargo run --example text_generation`

use orch::execution::*;

mod example_utils;
use example_utils::get_lm;

#[tokio::main]
async fn main() {
    let (lm, _) = get_lm();

    let prompt = "What is 2+2?";

    println!("Prompt: {prompt}");
    println!("---");

    let executor = TextExecutorBuilder::new()
        .with_lm(&*lm)
        .try_build()
        .unwrap();
    let response = executor.execute(prompt).await.expect("Execution failed");

    println!("Response:");
    println!("{}", response.content);

    assert!(response.content.contains('4'));
}
