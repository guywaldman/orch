//! This example demonstrates how to use the `Executor` to generate a structured response from the LLM.

use orch::execution::*;
use orch::lm::*;
use orch::response::*;

#[derive(OrchResponseOptions)]
pub enum BlogPostReviewerResponseOption {
    #[response(
        scenario = "You have reviewed the blog post",
        description = "Suggestions for improving the blog post"
    )]
    #[schema(
        field = "suggestions",
        description = "Suggestions for improving the blog post",
        example = "[\"You wrote 'excellent' in two consecutive paragraphs in section 'Introduction'\"]"
    )]
    Answer {
        #[allow(dead_code)]
        suggestions: Vec<String>,
    },
    #[response(
        scenario = "For some reason you failed to generate suggestions",
        description = "Reason why you failed to generate suggestions"
    )]
    #[schema(
        field = "reason",
        description = "Reason why you failed to generate suggestions",
        example = "Content was invalid"
    )]
    Fail {
        #[allow(dead_code)]
        reason: String,
    },
}

#[tokio::main]
async fn main() {
    // ! Change this to use a different provider.
    let provider = LanguageModelProvider::OpenAi;

    let prompt = "
		# Introduction
		Hello, I am Guy. This is my first blog post!
		";

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

    let executor = StructuredExecutorBuilder::new()
        .with_lm(&*lm)
        .with_preamble("You are an experienced writer and blog post reviewer who helps users improve their blog posts.")
        .with_options(&options!(BlogPostReviewerResponseOption))
        .try_build()
        .unwrap();
    let response = executor.execute(prompt).await.expect("Execution failed");

    println!("Response:");
    println!("{:?}", response.content);
}
