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
            "gpt-3.5-turbo-16k-0613",
            text_completion_config,
        )))
        .with_tool(
            ToolBuilder::default()
                .name("world_population")
                .description("Returns the world's population for a given year.")
                .examples(vec![(
                    "What was the population of the world at 2020?".to_owned(),
                    "7.8 billion people".to_owned(),
                )
                    .into()])
                .executor(ToolExecutor::Function(|name: &str| {
                    // TODO: Clean the input and remove the quotes.
                    match name.replace('\"', "").as_str() {
                        "2007" => Some("6.6 billion people".to_owned()),
                        _ => Some("404 billion people".to_owned()),
                    }
                }))
                .build()
                .unwrap(),
        )
        .with_tool(
            ToolBuilder::default()
                .name("calculator")
                .description("Calculate the result of a mathematical expression.")
                .examples(vec![("What is 3+2?".to_owned(), "5".to_owned()).into()])
                .executor(ToolExecutor::Function(|_: &str| Some("2007".to_owned())))
                .build()
                .unwrap(),
        )
        .build();

    let input = "What was the population of the world at 2000+3?";
    let run_result = agent.run(input).await;
    dbg!(&run_result);
}
