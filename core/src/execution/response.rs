#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseFormat {
    Text,
    Json,
}

impl Default for ResponseFormat {
    fn default() -> Self {
        Self::Text
    }
}
