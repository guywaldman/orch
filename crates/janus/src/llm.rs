use async_trait::async_trait;
use derive_builder::Builder;

#[derive(Debug, Default, Builder)]
#[builder(setter(into))]
pub struct TextCompletionConfig {
    pub max_tokens: usize,
    #[builder(default = "0.0")]
    pub temperature: f32,
}

#[async_trait]
pub trait TextCompletionLlm {
    async fn complete(&self, input: &str) -> String;
}

pub struct LlmBuilder {
    llm: Option<Box<dyn TextCompletionLlm>>,
}

pub mod third_party_llm {
    use super::*;
    use async_trait::async_trait;
    use openai::{completions::Completion, set_key};

    pub struct OpenAi<'a> {
        pub model: &'a str,
        config: TextCompletionConfig,
    }

    impl<'a> OpenAi<'a> {
        pub fn new(api_key: String, model: &'a str, config: TextCompletionConfig) -> Self {
            set_key(api_key);

            Self { model, config }
        }
    }

    #[async_trait]
    impl TextCompletionLlm for OpenAi<'_> {
        async fn complete(&self, input: &str) -> String {
            let completion = Completion::builder(self.model)
                .prompt(input)
                .max_tokens(self.config.max_tokens as u16)
                .temperature(self.config.temperature)
                .create()
                .await
                .unwrap();
            let result = completion.unwrap().choices.first().unwrap().text.clone();
            result
        }
    }
}
