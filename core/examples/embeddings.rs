//! This example demonstrates how to use the `Executor` to generate embeddings from the language model.
//! Run like so: `cargo run --example embeddings`

mod example_utils;
use example_utils::get_lm;

use orch::{execution::*, lm::LanguageModelProvider};

#[tokio::main]
async fn main() {
    let (lm, provider) = get_lm();

    if provider == LanguageModelProvider::Anthropic {
        println!("Anthropic does not have built-in embedding models. Skipping example.");
        return;
    }

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
