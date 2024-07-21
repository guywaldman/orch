use async_trait::async_trait;
use lm::{
    error::LanguageModelError,
    models::{
        TextCompleteOptions, TextCompleteResponse, TextCompleteStreamOptions,
        TextCompleteStreamResponse,
    },
    ollama_embedding_model, ollama_model, LanguageModel, LanguageModelProvider,
};
use net::SseClient;
use thiserror::Error;
use tokio_stream::StreamExt;

use crate::*;

use super::{
    OllamaApiModelsMetadata, OllamaEmbeddingsRequest, OllamaEmbeddingsResponse,
    OllamaGenerateRequest, OllamaGenerateResponse, OllamaGenerateStreamItemResponse,
};

#[derive(Debug, Clone)]
pub struct Ollama {
    pub base_url: String,
    pub model: String,
    pub embeddings_model: String,
}

impl Default for Ollama {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            model: ollama_model::CODESTRAL.to_string(),
            embeddings_model: ollama_embedding_model::NOMIC_EMBED_TEXT.to_string(),
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

impl Ollama {
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
        let url = format!("{}/{}", self.base_url, url);

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
}

#[async_trait]
impl LanguageModel for Ollama {
    async fn text_complete(
        &self,
        prompt: &str,
        system_prompt: &str,
        _options: TextCompleteOptions,
    ) -> Result<TextCompleteResponse, LanguageModelError> {
        let body = OllamaGenerateRequest {
            model: self.model.to_owned(),
            prompt: prompt.to_string(),
            system: Some(system_prompt.to_string()),
            ..Default::default()
        };

        let client = reqwest::Client::new();
        let url = format!("{}/api/generate", self.base_url);
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
            model: self.model.to_owned(),
            prompt: prompt.to_string(),
            stream: Some(true),
            format: None,
            images: None,
            system: Some(system_prompt.to_string()),
            keep_alive: Some("5m".to_string()),
            context: options.context,
        };

        let url = format!("{}/api/generate", self.base_url);
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
        let url = format!("{}/api/embeddings", self.base_url);
        let body = OllamaEmbeddingsRequest {
            model: self.embeddings_model.to_owned(),
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
        self.model.to_string()
    }

    fn embedding_model_name(&self) -> String {
        self.embeddings_model.to_string()
    }
}
