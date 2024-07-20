pub mod openai_model {
    pub const GPT_3_5_TURBO: &str = "gpt-3.5-turbo";
    pub const GPT_4: &str = "gpt-4";
    pub const GPT_4O_TURBO: &str = "gpt-4o-turbo";
    pub const GPT_4O_MINI: &str = "gpt-4o-mini";
}

pub mod openai_embedding_model {
    pub const TEXT_EMBEDDING_ADA_002: &str = "text-embedding-ada-002";
    pub const TEXT_EMBEDDING_ADA_002_DIMENSIONS: usize = 1536;
    pub const TEXT_EMBEDDING_3_SMALL: &str = "text-embedding-3-small";
    pub const TEXT_EMBEDDING_3_SMALL_DIMENSIONS: usize = 1536;
    pub const TEXT_EMBEDDING_3_LARGE: &str = "text-embedding-3-large";
    pub const TEXT_EMBEDDING_3_LARGE_DIMENSIONS: usize = 3072;
}
