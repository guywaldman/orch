use dotenv::dotenv;
use orch::builtins::tools;
use orch::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let text_completion_config = TextCompletionConfigBuilder::default()
        .max_tokens(1000_usize)
        .temperature(0.0)
        .build()
        .unwrap();

    let agent = AgentBuilder::new()
        .with_llm(Box::new(third_party_llm::OpenAi::new(
            api_key,
            "gpt-3.5-turbo-16k-0613",
            text_completion_config,
        )))
        .with_tool(tools::http_tool())
        .with_tool(tools::pdf_summary_tool())
        .with_tool(tools::web_search_tool())
        .build();

    let input = "Give me the results for the query \"What is the capital of France?\"";
    let run_result = agent.run(input).await;
    dbg!(&run_result);
}
