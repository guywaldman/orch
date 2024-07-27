//! This example demonstrates how to use the `Executor` to generate a structured response from the LLM.
//! Run like so: `cargo run --example structured_data_generation_blog -- blog.md`

#![allow(dead_code)]

use orch::alignment::AlignmentStrategyBuilder;
use orch::execution::*;
use orch::response::*;

mod example_utils;
use example_utils::get_lm;

#[derive(Variants, serde::Deserialize)]
#[serde(tag = "response_type")]
pub enum ResponseVariants {
    Answer(AnswerResponseVariant),
    Fail(FailResponseVariant),
}

#[derive(Variant, serde::Deserialize)]
#[variant(
    variant = "Answer",
    scenario = "You have reviewed the blog post",
    description = "Suggestions for improving the blog post"
)]
pub struct AnswerResponseVariant {
    #[schema(
        description = "Suggestions for improving the blog post",
        example = "[\"You wrote 'excellent' in two consecutive paragraphs in section 'Introduction'\"]"
    )]
    pub suggestions: Vec<String>,
}

#[derive(Variant, serde::Deserialize)]
#[variant(
    variant = "Fail",
    scenario = "For some reason you failed to generate suggestions",
    description = "Reason why you failed to generate suggestions"
)]
pub struct FailResponseVariant {
    #[schema(
        description = "Reason why you failed to generate suggestions",
        example = "Content was invalid"
    )]
    pub reason: String,
}

#[tokio::main]
async fn main() {
    let (lm, _) = get_lm();

    // In this example, we use the same LLM for the correction as for the main task.
    // This could be replaced by a smaller LM.
    let (corrector_lm, _) = get_lm();

    // We define an alignment strategy that uses the correction model.
    let alignment_strategy = AlignmentStrategyBuilder::new()
        .with_retries(2)
        .with_lm(&*corrector_lm)
        .try_build()
        .unwrap();

    // Mock blog post
    let prompt = "
        This is a blog post about the importance of blogging.

        # Introduction

        Blogging is a crucial skill for any writer. It allows you to share your thoughts and ideas with others, and it can help you build a following and establish yourself as an expert in your field.
    ";

    let executor = StructuredExecutorBuilder::new()
        .with_lm(&*lm)
        .with_preamble("
            You are an experienced writer and blog post reviewer who helps users improve their blog posts.
            You will receive a blog post written in Markdown, and you will need to provide suggestions for improving it.
            Provide *specific* suggestions for improving the blog post, these can as nitpicky as you want.
            Consider things such as grammar, spelling, clarity, and conciseness.
            Even things like mentioning the same phrase too much in one paragraph, etc.
            The tone should be personal, friendly and professional at the same time.

            Be very specific and refer to specific sentences, paragraph and sections of the blog post.
        ")
        .with_options(Box::new(variants!(ResponseVariants)))
        .with_alignment(alignment_strategy)
        .try_build()
        .unwrap();
    let response = executor.execute(prompt).await.expect("Execution failed");

    match response.content {
        ResponseVariants::Answer(answer) => {
            assert!(!answer.suggestions.is_empty());

            println!("Suggestions for improving the blog post:");
            for suggestion in answer.suggestions {
                println!("- {}", suggestion);
            }
        }
        ResponseVariants::Fail(fail) => {
            eprintln!("Model failed to generate a response: {}", fail.reason);
            std::process::exit(1);
        }
    }
}
