use crate::{Tool, ToolBuilder, ToolRunExample, ToolParams, ToolExecutor};

pub fn http_tool() -> Tool {
    ToolBuilder::default()
        .name("http")
        .description("Performs an HTTP request")
        .examples(vec![(
            "What is the contents of https://example.com/api/todos/2".to_owned(),
            "{ \"userId\": 1, \"id\": 2, \"title\": \"quis ut nam facilis et officia qui\", \"completed\": false }".to_owned(),
				).into()])
				.parameter_names(vec!["url".to_owned(), "method".to_owned(), "body".to_owned()])
				.parameter_examples(vec![
						vec!["https://example.com/api/todos/2".to_owned(), "GET".to_owned(), "".to_owned()],
						vec!["https://example.com/api/todos/2".to_owned(), "POST".to_owned(), "{ \"userId\": 1, \"id\": 2, \"title\": \"quis ut nam facilis et officia qui\", \"completed\": false }".to_owned()],
				])
				.executor(ToolExecutor::Code(|params: ToolParams| Box::pin(async move {
						let url = params.get("url").unwrap();
						let method = params.get("method").unwrap_or(&"GET".to_owned()).to_owned();
						let body = params.get("body").unwrap_or(&"".to_owned()).to_owned();
						let client = reqwest::Client::new();
						let response = match method.as_str() {
								"GET" => client.get(url).send().await.unwrap(),
								"POST" => client.post(url).body(body.to_owned()).send().await.unwrap(),
								_ => panic!("Unsupported method: {}", method),
						};
						let text = response.text().await.unwrap();
						Some(text)
				})))
        .build()
				.expect("Failed to build tool")
}