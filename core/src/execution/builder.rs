use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutorBuilderError {
    #[error("Configuration error: {0} is not set")]
    ConfigurationNotSet(String),
}
