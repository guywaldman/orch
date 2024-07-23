use serde::{Deserialize, Serialize};

use crate::lm::LanguageModel;

use super::{Ollama, OpenAi};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LanguageModelProvider {
    #[serde(rename = "ollama")]
    Ollama,
    #[serde(rename = "openai")]
    OpenAi,
}

impl LanguageModelProvider {
    /// Returns whether the provider runs local inference or not.
    pub fn is_local(&self) -> bool {
        match self {
            LanguageModelProvider::Ollama => false,
            LanguageModelProvider::OpenAi => true,
        }
    }
}

pub enum OrchLanguageModel {
    Ollama(Ollama),
    OpenAi(OpenAi),
}

impl OrchLanguageModel {
    pub fn provider(&self) -> LanguageModelProvider {
        match self {
            OrchLanguageModel::Ollama(_) => LanguageModelProvider::Ollama,
            OrchLanguageModel::OpenAi(_) => LanguageModelProvider::OpenAi,
        }
    }

    pub fn text_completion_model_name(&self) -> String {
        match self {
            OrchLanguageModel::Ollama(lm) => lm.text_completion_model_name(),
            OrchLanguageModel::OpenAi(lm) => lm.text_completion_model_name(),
        }
    }

    pub fn embedding_model_name(&self) -> String {
        match self {
            OrchLanguageModel::Ollama(lm) => lm.embedding_model_name(),
            OrchLanguageModel::OpenAi(lm) => lm.embedding_model_name(),
        }
    }
}
