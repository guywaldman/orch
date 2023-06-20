use dotenv::dotenv;
use janus::*;

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
            "text-davinci-003",
            text_completion_config,
        )))
        .build();

    println!("{}", agent.prompt());

    let run_result = agent.run("How are you?").await;
    dbg!(&run_result);
}
