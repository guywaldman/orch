//! This example demonstrates how to use the `Executor` to generate embeddings from the language model.
//! We construct an `Ollama` instance and use it to generate embeddings.

use orch::execution::*;
use orch::lm::*;

#[tokio::main]
async fn main() {
    // ! Change this to use a different provider.
    let provider = LanguageModelProvider::Ollama;

    let text = "Lorem ipsum";

    println!("Text: {text}");
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
                .with_embeddings_model(ollama_embedding_model::NOMIC_EMBED_TEXT.to_string())
                .try_build()
                .unwrap(),
        ),
        LanguageModelProvider::OpenAi => Box::new(
            OpenAiBuilder::new()
                .with_api_key(open_ai_api_key)
                .try_build()
                .unwrap(),
        ),
    };

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
