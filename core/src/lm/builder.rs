use thiserror::Error;

use super::LanguageModel;

#[derive(Debug, Error)]
pub enum LanguageModelBuilderError {
    #[error("{0} is not set")]
    ConfigurationNotSet(String),
}

pub trait LanguageModelBuilder<T: LanguageModel> {
    fn new() -> Self;
    fn try_build(self) -> Result<T, LanguageModelBuilderError>;
}
