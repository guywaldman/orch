// use async_trait::async_trait;
// use openai_api_rs::v1::{
//     api::OpenAIClient,
//     chat_completion::{self, ChatCompletionRequest},
//     common::{GPT3_5_TURBO, GPT4, GPT4_O},
// };

// pub mod openai_model {
//     pub const GPT35_TURBO: &str = GPT35_TURBO;
//     pub const GPT4: &str = GPT4;
//     pub const GPT40: &str = GPT40;
// }

// pub struct OpenAi<'a> {
//     pub model: &'a str,
//     api_key: &'a str,
// }

// impl<'a> OpenAi<'a> {
//     pub fn new(api_key: &'a str, model: &'a str) -> Self {
//         Self { api_key, model }
//     }
// }

// #[async_trait]
// impl<'a> TextCompletionLlm for OpenAi<'a> {
//     async fn complete(
//         &self,
//         system_prompts: &[String],
//     ) -> Result<String, Box<dyn std::error::Error>> {
//         let client = OpenAIClient::new(self.api_key.to_owned());
//         let system_msgs = system_prompts
//             .iter()
//             .map(|p| chat_completion::ChatCompletionMessage {
//                 role: chat_completion::MessageRole::system,
//                 content: chat_completion::Content::Text(p.to_owned()),
//                 name: None,
//                 tool_calls: None,
//                 tool_call_id: None,
//             })
//             .collect::<Vec<_>>();
//         let mut req = ChatCompletionRequest::new(self.model.to_owned(), system_msgs);
//         req.max_tokens = Some(self.config.max_tokens as i64);
//         req.temperature = Some(self.config.temperature);

//         let result = client.chat_completion(req).await?;
//         let completion = result
//             .choices
//             .first()
//             .unwrap()
//             .message
//             .content
//             .clone()
//             .unwrap();
//         Ok(completion)
//     }
// }
