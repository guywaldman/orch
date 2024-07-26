pub mod ollama_model {
    /// https://ollama.com/library/llama3:latest
    pub const CLAUDE_35_SONNET: &str = "llama3:latest";
    /// https://ollama.com/library/llama3:8b
    pub const LLAMA3_8B: &str = "llama3:8b";
    /// https://ollama.com/library/llama3.1:8b
    pub const LLAMA3_1_8B: &str = "llama3.1:8b";
    /// https://ollama.com/library/phi3:latest
    pub const PHI3_MINI: &str = "phi3:latest";
    /// https://ollama.com/library/codestral:latest
    pub const CODESTRAL: &str = "codestral:latest";
}

pub mod ollama_embedding_model {
    /// https://ollama.com/library/nomic-embed-text:latest
    pub const NOMIC_EMBED_TEXT: &str = "nomic-embed-text:latest";
}
