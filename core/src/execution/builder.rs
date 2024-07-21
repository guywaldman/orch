use orch_response::OrchResponseVariants;
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
pub struct StructuredExecutorBuilder<'a, T> {
    lm: Option<&'a dyn LanguageModel>,
    preamble: Option<&'a str>,
    variants: Option<&'a dyn OrchResponseVariants<T>>,
}

impl<'a, T> StructuredExecutorBuilder<'a, T> {
    pub fn new() -> Self {
        Self {
            lm: None,
            preamble: None,
            variants: None,
        }
    }

    pub fn with_lm(mut self, lm: &'a dyn LanguageModel) -> Self {
        self.lm = Some(lm);
        self
    }

    pub fn with_options(mut self, options: &'a dyn OrchResponseVariants<T>) -> Self {
        self.variants = Some(options);
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
        let Some(response_options) = self.variants else {
            return Err(ExecutorBuilderError::ConfigurationNotSet(
                "Response options".to_string(),
            ));
        };
        Ok(StructuredExecutor {
            lm,
            preamble: self.preamble,
            variants: response_options,
        })
    }
}
