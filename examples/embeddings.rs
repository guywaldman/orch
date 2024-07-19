//! This example demonstrates how to use the `Executor` to generate embeddings from the LLM.
//! We construct an `Ollama` instance and use it to generate embeddings.
//!
use orch::{Executor, OllamaBuilder};

#[tokio::main]
async fn main() {
    let text = "Lorem ipsum";

    println!("Text: {text}");
    println!("---");

    let ollama = OllamaBuilder::new().build();
    let executor = Executor::new(&ollama);
    let embedding = executor
        .generate_embedding(text)
        .await
        .expect("Execution failed");

    println!("Embedding:");
    println!("{:?}", embedding);
}
