use async_trait::async_trait;
use openai_api_rs::v1::{
    api::OpenAIClient,
    chat_completion::{self, ChatCompletionRequest},
    common::{GPT3_5_TURBO, GPT4, GPT4_O},
};

use crate::{TextCompletionConfig, TextCompletionLlm};

#[derive(Debug, Clone)]

pub enum OpenAiModel {
    Gpt35Turbo,
    Gpt4,
    Gpt40,
}

pub struct OpenAi {
    pub model: OpenAiModel,
    api_key: String,
    config: TextCompletionConfig,
}

impl OpenAi {
    pub fn new(api_key: String, model: OpenAiModel, config: TextCompletionConfig) -> Self {
        Self {
            api_key,
            model,
            config,
        }
    }

    pub fn model_name(model: &OpenAiModel) -> String {
        match model {
            OpenAiModel::Gpt35Turbo => GPT3_5_TURBO.to_string(),
            OpenAiModel::Gpt4 => GPT4.to_string(),
            OpenAiModel::Gpt40 => GPT4_O.to_string(),
        }
    }
}

#[async_trait]
impl TextCompletionLlm for OpenAi {
    async fn complete(
        &self,
        system_prompts: &[String],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = OpenAIClient::new(self.api_key.clone());
        let system_msgs = system_prompts
            .iter()
            .map(|p| chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::system,
                content: chat_completion::Content::Text(p.to_owned()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            })
            .collect::<Vec<_>>();
        let mut req = ChatCompletionRequest::new(Self::model_name(&self.model), system_msgs);
        req.max_tokens = Some(self.config.max_tokens as i64);
        req.temperature = Some(self.config.temperature);

        let result = client.chat_completion(req).await?;
        let completion = result
            .choices
            .first()
            .unwrap()
            .message
            .content
            .clone()
            .unwrap();
        Ok(completion)
    }
}
