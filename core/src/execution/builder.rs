use orch_response::ResponseOptions;
use thiserror::Error;

use crate::lm::LanguageModel;

use super::{StructuredExecutor, TextExecutor};

#[derive(Debug, Error)]
pub enum ExecutorBuilderError {
    #[error("Configuration error: {0} is not set")]
    ConfigurationNotSet(String),
}

#[derive(Default)]
pub struct TextExecutorBuilder<'a> {
    lm: Option<&'a dyn LanguageModel>,
    preamble: Option<&'a str>,
}

impl<'a> TextExecutorBuilder<'a> {
    pub fn new() -> Self {
        Self {
            lm: None,
            preamble: None,
        }
    }

    pub fn with_lm(mut self, lm: &'a dyn LanguageModel) -> Self {
        self.lm = Some(lm);
        self
    }

    pub fn with_preamble(mut self, preamble: &'a str) -> Self {
        self.preamble = Some(preamble);
        self
    }

    pub fn try_build(self) -> Result<TextExecutor<'a>, ExecutorBuilderError> {
        let Some(lm) = self.lm else {
            return Err(ExecutorBuilderError::ConfigurationNotSet(
                "Language model".to_string(),
            ));
        };
        Ok(TextExecutor {
            lm,
            preamble: self.preamble,
        })
    }
}

#[derive(Default)]
pub struct StructuredExecutorBuilder<'a, T>
where
    T: serde::de::DeserializeOwned + Sized,
{
    lm: Option<&'a dyn LanguageModel>,
    preamble: Option<&'a str>,
    options: Option<&'a dyn ResponseOptions<T>>,
}

impl<'a, T> StructuredExecutorBuilder<'a, T>
where
    T: serde::de::DeserializeOwned + Sized,
{
    pub fn new() -> Self {
        Self {
            lm: None,
            preamble: None,
            options: None,
        }
    }

    pub fn with_lm(mut self, lm: &'a dyn LanguageModel) -> Self {
        self.lm = Some(lm);
        self
    }

    pub fn with_options(mut self, options: &'a dyn ResponseOptions<T>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn with_preamble(mut self, preamble: &'a str) -> Self {
        self.preamble = Some(preamble);
        self
    }

    pub fn try_build(self) -> Result<StructuredExecutor<'a, T>, ExecutorBuilderError> {
        let Some(lm) = self.lm else {
            return Err(ExecutorBuilderError::ConfigurationNotSet(
                "Language model".to_string(),
            ));
        };
        let Some(response_options) = self.options else {
            return Err(ExecutorBuilderError::ConfigurationNotSet(
                "Response options".to_string(),
            ));
        };
        Ok(StructuredExecutor {
            lm,
            preamble: self.preamble,
            response_options,
        })
    }
}
