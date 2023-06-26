use dotenv::dotenv;
use janus::builtins::tools;
use janus::*;
use tokio::task;

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
        .build();

    let input = "What's the title of the todo in https://jsonplaceholder.typicode.com/todos/2, followed by the title of the todo in https://jsonplaceholder.typicode.com/todos/3?";
    let run_result = agent.run(input).await;
    dbg!(&run_result);
}
