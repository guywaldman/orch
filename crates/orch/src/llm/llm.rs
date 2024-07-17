use async_trait::async_trait;
use derive_builder::Builder;


#[derive(Debug, Default, Builder)]
#[builder(setter(into))]
pub struct TextCompletionConfig {
    pub max_tokens: usize,
    #[builder(default = "0.0")]
    pub temperature: f64
}

#[async_trait]
pub trait TextCompletionLlm {
    async fn complete(&self, messages: &[String]) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct LlmBuilder {
    llm: Option<Box<dyn TextCompletionLlm>>,
}