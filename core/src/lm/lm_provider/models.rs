use serde::{Deserialize, Serialize};

use crate::lm::{LanguageModel, LanguageModelProviderError};

use super::{Anthropic, Ollama, OpenAi};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LanguageModelProvider {
    #[serde(rename = "ollama")]
    Ollama,
    #[serde(rename = "openai")]
    OpenAi,
    #[serde(rename = "anthropic")]
    Anthropic,
}

impl TryFrom<&str> for LanguageModelProvider {
    type Error = LanguageModelProviderError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "ollama" => LanguageModelProvider::Ollama,
            "openai" => LanguageModelProvider::OpenAi,
            "anthropic" => LanguageModelProvider::Anthropic,
            _ => return Err(LanguageModelProviderError::InvalidValue(value.to_string())),
        })
    }
}

impl LanguageModelProvider {
    /// Returns whether the provider runs local inference or not.
    pub fn is_local(&self) -> bool {
        match self {
            LanguageModelProvider::Ollama => true,
            LanguageModelProvider::OpenAi => false,
            LanguageModelProvider::Anthropic => false,
        }
    }
}

pub enum OrchLanguageModel {
    Ollama(Ollama),
    OpenAi(OpenAi),
    Anthropic(Anthropic),
}

impl OrchLanguageModel {
    pub fn provider(&self) -> LanguageModelProvider {
        match self {
            OrchLanguageModel::Ollama(_) => LanguageModelProvider::Ollama,
            OrchLanguageModel::OpenAi(_) => LanguageModelProvider::OpenAi,
            OrchLanguageModel::Anthropic(_) => LanguageModelProvider::Anthropic,
        }
    }

    pub fn text_completion_model_name(&self) -> String {
        match self {
            OrchLanguageModel::Ollama(lm) => lm.text_completion_model_name(),
            OrchLanguageModel::OpenAi(lm) => lm.text_completion_model_name(),
            OrchLanguageModel::Anthropic(lm) => lm.text_completion_model_name(),
        }
    }

    pub fn embedding_model_name(&self) -> String {
        match self {
            OrchLanguageModel::Ollama(lm) => lm.embedding_model_name(),
            OrchLanguageModel::OpenAi(lm) => lm.embedding_model_name(),
            OrchLanguageModel::Anthropic(lm) => lm.embedding_model_name(),
        }
    }
}
