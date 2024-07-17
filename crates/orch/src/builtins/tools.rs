use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serpapi_search_rust::serp_api_search::SerpApiSearch;

use crate::{Tool, ToolBuilder, ToolExecutor, ToolParams};

#[derive(Debug, Serialize, Deserialize)]
struct SerpApiSearchResponse {
    #[serde(rename = "organic_results")]
    organic_results: Vec<SerpApiSearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SerpApiSearchResult {
    title: Option<String>,
    link: Option<String>,
    snippet: Option<String>,
}

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

pub fn pdf_summary_tool() -> Tool {
    ToolBuilder::default()
			.name("summarize_pdf")
			.description("Given a URL to a PDF, returns a summary of the PDF")
			.examples(vec![(
					"What is the summary of https://arxiv.org/pdf/2106.01401.pdf".to_owned(),
					"{ \"summary\": \"Deep learning has been successfully applied to many tasks in natural language processing, including question answering, machine translation, and document summarization. However, these models are typically trained on large datasets of human-labeled examples. In this paper, we explore the possibility of training a summarization model from a corpus of unlabelled documents, using only a small number of human-written summaries as input. We propose a simple yet effective method for fine-tuning a pretrained language model on a document summarization dataset, and show that it outperforms a number of strong baselines on the CNN/Daily Mail dataset. We also introduce a new dataset of scientific papers called SciTLDR, consisting of 5,000 expert-written summaries of 5,000 scientific papers from multiple disciplines. We show that our method achieves strong performance on this dataset as well.\" }".to_owned(),
			).into()])
			.parameter_names(vec!["url".to_owned()])
			.parameter_examples(vec![
					vec!["https://arxiv.org/pdf/2106.01401.pdf".to_owned()],
					vec!["https://gov.uk/government/publications/uk-trade-tariff-eu-referendum-result/uk-trade-tariff.pdf".to_owned()],
			])
			.executor(ToolExecutor::Code(|params: ToolParams| Box::pin(async move {
					let url = params.get("url").unwrap();
					let client = reqwest::Client::new();
					let response = client.request(reqwest::Method::GET, url).header("Accept", "application/pdf").send().await.unwrap();
					let buffer = response.bytes().await.unwrap();
					let text = pdf_extract::extract_text_from_mem(buffer.as_ref()).unwrap();
					Some(text)
			})))
			.build()
			.expect("Failed to build tool")
}

async fn web_search_results(query: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    params.insert("q".to_string(), query.to_string());
    params.insert("hl".to_string(), "en".to_string());
    params.insert("gl".to_string(), "us".to_string());
    params.insert("google_domain".to_string(), "google.com".to_string());

    let api_key = std::env::var("SERP_API_KEY").expect("SERP_API_KEY not set");
    let search = SerpApiSearch::google(params, api_key);
    let results = search.json().await?;
    let parsed_results = serde_json::from_value::<SerpApiSearchResponse>(results)?;
    let results = parsed_results.organic_results;

    let results_json = serde_json::to_string(&results)?;
    Ok(vec![results_json])
}

pub fn web_search_tool() -> Tool {
    ToolBuilder::default()
			.name("web_search")
			.description("Given a query, returns the top search result from a search engine")
			.examples(vec![(
					"What are the top search results for 'hotels in New York'".to_owned(),
					"{\"results\": [{\"title\": \"AllRecipes\",\"url\": \"https://www.allrecipes.com\"},{\"title\": \"Food Network\",\"url\": \"https://www.foodnetwork.com\"},{\"title\": \"Epicurious\",\"url\": \"https://www.epicurious.com\"},{\"title\": \"BBC Good Food\",\"url\": \"https://www.bbcgoodfood.com\"},{\"title\": \"Tasty\",\"url\": \"https://www.tasty.co\"}]}	".to_owned(),
			).into()])
			.parameter_names(vec!["query".to_owned()])
			.parameter_examples(vec![
				vec!["cooking recipes".to_owned()],
				vec!["places to visit in London".to_owned()]
			])
			.executor(ToolExecutor::Code(|params: ToolParams| Box::pin(async move {
					let query = params.get("query").unwrap();
					let results = web_search_results(query).await.unwrap();
					Some(results.join("\n"))
			})))
			.build()
			.expect("Failed to build tool")
}
