use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutorBuilderError {
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("{0} is not set")]
    ConfigurationNotSet(String),
}
