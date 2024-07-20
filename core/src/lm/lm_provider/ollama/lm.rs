use lm::{
    error::LanguageModelError,
    models::{
        TextCompleteOptions, TextCompleteResponse, TextCompleteStreamOptions,
        TextCompleteStreamResponse,
    },
    LanguageModel, LanguageModelProvider,
};
use net::SseClient;
use thiserror::Error;
use tokio_stream::StreamExt;

use crate::*;

use super::{
    OllamaApiModelsMetadata, OllamaEmbeddingsRequest, OllamaEmbeddingsResponse,
    OllamaGenerateRequest, OllamaGenerateResponse, OllamaGenerateStreamItemResponse,
};

pub mod ollama_model {
    pub const CODESTRAL: &str = "codestral:latest";
}

pub mod ollama_embedding_model {
    pub const NOMIC_EMBED_TEXT: &str = "nomic-embed-text:latest";
}

#[derive(Debug, Clone)]
pub struct Ollama<'a> {
    base_url: &'a str,
    pub model: Option<&'a str>,
    pub embeddings_model: Option<&'a str>,
}

impl Default for Ollama<'_> {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434",
            model: Some(ollama_model::CODESTRAL),
            embeddings_model: Some(ollama_embedding_model::NOMIC_EMBED_TEXT),
        }
    }
}

pub struct OllamaBuilder<'a> {
    base_url: &'a str,
    model: Option<&'a str>,
    embeddings_model: Option<&'a str>,
}

impl Default for OllamaBuilder<'_> {
    fn default() -> Self {
        let ollama = Ollama::default();
        Self {
            base_url: ollama.base_url,
            model: ollama.model,
            embeddings_model: ollama.embeddings_model,
        }
    }
}

impl<'a> OllamaBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_base_url(mut self, base_url: &'a str) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn with_model(mut self, model: &'a str) -> Self {
        self.model = Some(model);
        self
    }

    pub fn with_embeddings_model(mut self, embeddings_model: &'a str) -> Self {
        self.embeddings_model = Some(embeddings_model);
        self
    }

    pub fn build(self) -> Ollama<'a> {
        Ollama {
            base_url: self.base_url,
            model: self.model,
            embeddings_model: self.embeddings_model,
        }
    }
}

#[derive(Error, Debug)]
pub enum OllamaError {
    #[error("Unexpected response from API. Error: {0}")]
    Api(String),

    #[error("Unexpected error when parsing response from Ollama. Error: {0}")]
    Parsing(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error(
        "Ollama API is not available. Please check if Ollama is running in the specified port. Error: {0}"
    )]
    ApiUnavailable(String),
}

impl<'a> Ollama<'a> {
    /// Lists the running models in the Ollama API.
    ///
    /// # Returns
    ///
    /// A [Result] containing the list of running models or an error if there was a problem.
    ///
    #[allow(dead_code)]
    pub(crate) fn list_running_models(&self) -> Result<OllamaApiModelsMetadata, OllamaError> {
        let response = self.get_from_ollama_api("api/ps")?;
        let parsed_response = Self::parse_models_response(&response)?;
        Ok(parsed_response)
    }

    // /// Lists the local models in the Ollama API.
    // ///
    // /// # Returns
    // ///
    // /// A [Result] containing the list of local models or an error if there was a problem.
    #[allow(dead_code)]
    pub fn list_local_models(&self) -> Result<OllamaApiModelsMetadata, OllamaError> {
        let response = self.get_from_ollama_api("api/tags")?;
        let parsed_response = Self::parse_models_response(&response)?;
        Ok(parsed_response)
    }

    fn parse_models_response(response: &str) -> Result<OllamaApiModelsMetadata, OllamaError> {
        let models: OllamaApiModelsMetadata =
            serde_json::from_str(response).map_err(|e| OllamaError::Parsing(e.to_string()))?;
        Ok(models)
    }

    fn get_from_ollama_api(&self, url: &str) -> Result<String, OllamaError> {
        let url = format!("{}/{}", self.base_url()?, url);

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(url)
            .send()
            .map_err(|e| OllamaError::ApiUnavailable(e.to_string()))?;
        let response_text = response
            .text()
            .map_err(|e| OllamaError::Api(e.to_string()))?;
        Ok(response_text)
    }

    fn base_url(&self) -> Result<String, OllamaError> {
        Ok(self.base_url.to_string())
    }

    fn model(&self) -> Result<String, OllamaError> {
        self.model
            .map(|s| s.to_owned())
            .ok_or_else(|| OllamaError::Configuration("Model not set".to_string()))
    }

    fn embedding_model(&self) -> Result<String, OllamaError> {
        self.embeddings_model
            .map(|s| s.to_owned())
            .ok_or_else(|| OllamaError::Configuration("Embedding model not set".to_string()))
    }
}

impl<'a> LanguageModel for Ollama<'a> {
    async fn text_complete(
        &self,
        prompt: &str,
        system_prompt: &str,
        _options: TextCompleteOptions,
    ) -> Result<TextCompleteResponse, LanguageModelError> {
        let body = OllamaGenerateRequest {
            model: self
                .model()
                .map_err(|_e| LanguageModelError::Configuration("Model not set".to_string()))?,
            prompt: prompt.to_string(),
            system: Some(system_prompt.to_string()),
            ..Default::default()
        };

        let client = reqwest::Client::new();
        let url = format!(
            "{}/api/generate",
            self.base_url()
                .map_err(|_e| LanguageModelError::Configuration("Base URL not set".to_string()))?
        );
        let response = client
            .post(url)
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .await
            .map_err(|e| LanguageModelError::Ollama(OllamaError::ApiUnavailable(e.to_string())))?;
        let body = response
            .text()
            .await
            .map_err(|e| LanguageModelError::Ollama(OllamaError::Api(e.to_string())))?;
        let ollama_response: OllamaGenerateResponse = serde_json::from_str(&body)
            .map_err(|e| LanguageModelError::Ollama(OllamaError::Parsing(e.to_string())))?;
        let response = TextCompleteResponse {
            text: ollama_response.response,
            context: ollama_response.context,
        };
        Ok(response)
    }

    async fn text_complete_stream(
        &self,
        prompt: &str,
        system_prompt: &str,
        options: TextCompleteStreamOptions,
    ) -> Result<TextCompleteStreamResponse, LanguageModelError> {
        let body = OllamaGenerateRequest {
            model: self.model()?,
            prompt: prompt.to_string(),
            stream: Some(true),
            format: None,
            images: None,
            system: Some(system_prompt.to_string()),
            keep_alive: Some("5m".to_string()),
            context: options.context,
        };

        let url = format!("{}/api/generate", self.base_url()?);
        let stream = SseClient::post(&url, Some(serde_json::to_string(&body).unwrap()));
        let stream = stream.map(|event| {
            let parsed_message = serde_json::from_str::<OllamaGenerateStreamItemResponse>(&event);
            match parsed_message {
                Ok(message) => Ok(message.response),
                Err(e) => Err(LanguageModelError::Ollama(OllamaError::Parsing(
                    e.to_string(),
                ))),
            }
        });
        let response = TextCompleteStreamResponse {
            stream: Box::pin(stream),
        };
        Ok(response)
    }

    async fn generate_embedding(&self, prompt: &str) -> Result<Vec<f32>, LanguageModelError> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/embeddings", self.base_url()?);
        let body = OllamaEmbeddingsRequest {
            model: self.embedding_model()?,
            prompt: prompt.to_string(),
        };
        let response = client
            .post(url)
            .body(
                serde_json::to_string(&body)
                    .map_err(|e| OllamaError::Serialization(e.to_string()))?,
            )
            .send()
            .await
            .map_err(|e| OllamaError::ApiUnavailable(e.to_string()))?;
        let body = response
            .text()
            .await
            .map_err(|e| OllamaError::Api(e.to_string()))?;
        let response: OllamaEmbeddingsResponse =
            serde_json::from_str(&body).map_err(|e| OllamaError::Parsing(e.to_string()))?;

        Ok(response.embedding)
    }

    fn provider(&self) -> LanguageModelProvider {
        LanguageModelProvider::Ollama
    }

    fn text_completion_model_name(&self) -> String {
        self.model().expect("Model not set").to_string()
    }

    fn embedding_model_name(&self) -> String {
        self.embedding_model()
            .expect("Embedding model not set")
            .to_string()
    }
}
