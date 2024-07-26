//! This example demonstrates how to use the `Executor` to generate embeddings from the language model.
//! Run like so: `cargo run --example embeddings`

mod example_utils;
use example_utils::get_lm;

use orch::execution::*;
use orch::lm::*;

// ! Change this to use a different provider.
pub const PROVIDER: LanguageModelProvider = LanguageModelProvider::Ollama;

#[tokio::main]
async fn main() {
    let lm = get_lm(PROVIDER);

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
