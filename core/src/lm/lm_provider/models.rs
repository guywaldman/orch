use serde::{Deserialize, Serialize};

use super::{Ollama, OpenAi};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LanguageModelProvider {
    #[serde(rename = "ollama")]
    Ollama,
    #[serde(rename = "openai")]
    OpenAi,
}

pub enum LanguageModelImplementation<'a> {
    Ollama(Ollama<'a>),
    OpenAi(OpenAi<'a>),
}
