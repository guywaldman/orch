//! This example demonstrates how to use the `Executor` to generate embeddings from the language model.
//! Run like so: `cargo run --example embeddings`

mod example_utils;
use example_utils::get_lm;

use orch::execution::*;

#[tokio::main]
async fn main() {
    let lm = get_lm(
        std::env::args()
            .nth(1)
            .unwrap_or("ollama".to_owned())
            .as_str(),
    );

    let text = "Lorem ipsum";

    println!("Text: {text}");
    println!("---");

    let executor = TextExecutorBuilder::new()
        .with_lm(&*lm)
        .try_build()
        .unwrap();
    let embedding = executor
        .generate_embedding(text)
        .await
        .expect("Execution failed");

    println!("Embedding:");
    println!("{:?}", embedding);
}
