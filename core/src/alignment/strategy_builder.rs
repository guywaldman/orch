use thiserror::Error;

use crate::lm::LanguageModel;

use super::strategy::AlignmentStrategy;

/// The default number of retries for the alignment strategy, if not overriden.
pub const DEFAULT_RETRIES: usize = 2;

#[derive(Debug, Error)]
pub enum AlignmentStrategyBuilderError {
    #[error("Configuration error: {0} is not set")]
    ConfigurationNotSet(String),
}

#[derive(Default)]
pub struct AlignmentStrategyBuilder {
    lm: Option<Box<dyn LanguageModel>>,
    retries: Option<usize>,
}

impl AlignmentStrategyBuilder {
    /// Creates a new `AlignmentStrategyBuilder` instance.
    pub fn new() -> Self {
        Self {
            lm: None,
            retries: Some(DEFAULT_RETRIES),
        }
    }

    /// Sets the language model to use for the alignment strategy.
    pub fn with_lm(mut self, lm: Box<dyn LanguageModel>) -> Self {
        self.lm = Some(lm);
        self
    }

    /// Sets the number of retries for the alignment strategy.
    pub fn with_retries(mut self, retries: usize) -> Self {
        self.retries = Some(retries);
        self
    }

    /// Builds the alignment strategy.
    /// May fail with a [`AlignmentStrategyBuilderErrro`] if some required configurations are not set.
    pub fn try_build(self) -> Result<AlignmentStrategy, AlignmentStrategyBuilderError> {
        let Some(lm) = self.lm else {
            return Err(AlignmentStrategyBuilderError::ConfigurationNotSet(
                "Language model".to_string(),
            ));
        };
        let Some(retries) = self.retries else {
            return Err(AlignmentStrategyBuilderError::ConfigurationNotSet(
                "Retries".to_string(),
            ));
        };
        Ok(AlignmentStrategy { lm, retries })
    }
}
