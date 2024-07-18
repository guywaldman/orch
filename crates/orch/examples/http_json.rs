// use dotenv::dotenv;
// use orch::builtins::tools;
// use orch::*;

#[tokio::main]
async fn main() {
    //     dotenv().ok();

    //     let api_key = std::env::var("OPENAI_API_KEY").unwrap();

    //     let text_completion_config = TextCompletionConfigBuilder::default()
    //         .max_tokens(1000_usize)
    //         .temperature(0.0)
    //         .build()
    //         .unwrap();

    //     let agent = AgentBuilder::new()
    //         .with_llm(Box::new(openai::OpenAi::new(
    //             api_key,
    //             openai::OpenAiModel::Gpt35Turbo,
    //             text_completion_config,
    //         )))
    //         .with_tool(tools::http_tool())
    //         .with_tool(tools::pdf_summary_tool())
    //         .build();

    //     let input = "Summarize https://scrumguides.org/docs/scrumguide/v2020/2020-Scrum-Guide-US.pdf";
    //     let run_result = agent.run(input).await;
    //     dbg!(&run_result);
}
