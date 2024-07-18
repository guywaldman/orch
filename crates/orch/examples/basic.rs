use std::pin;

use orch::{Llm, OllamaBuilder, TextCompleteStreamOptions};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let ollama = OllamaBuilder::new().build();
    let text_completion = ollama
        .text_complete_stream(
            "How are you?",
            "You are a helpful assistant",
            TextCompleteStreamOptions::default(),
        )
        .unwrap();

    let mut stream = text_completion.stream;
    while let Some(chunk) = stream.next().await {
        println!("{chunk}");
    }
}
