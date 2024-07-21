//! This example demonstrates how to use the `Executor` to generate a streaming response from the LLM.
//! We construct an `Ollama` instance and use it to generate a streaming response.

use orch::execution::*;
use orch::lm::*;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    // ! Change this to use a different provider.
    let provider = LanguageModelProvider::Ollama;

    let prompt = "What is 2+2?";

    println!("Prompt: {prompt}");
    println!("---");

    // Use a different language model, per the `provider` variable (feel free to change it).
    let open_ai_api_key = {
        if provider == LanguageModelProvider::OpenAi {
            std::env::var("OPENAI_API_KEY")
                .unwrap_or_else(|_| panic!("OPENAI_API_KEY environment variable not set"))
        } else {
            String::new()
        }
    };
    let lm: Box<dyn LanguageModel> = match provider {
        LanguageModelProvider::Ollama => Box::new(
            OllamaBuilder::new()
                .with_model(ollama_model::PHI3_MINI)
                .try_build()
                .unwrap(),
        ),
        LanguageModelProvider::OpenAi => Box::new(
            OpenAiBuilder::new()
                .with_api_key(&open_ai_api_key)
                .try_build()
                .unwrap(),
        ),
    };

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
