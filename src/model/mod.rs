use serde::{Deserialize, Serialize};

pub mod embedding;
pub mod llm;

// === Structures ===
#[derive(Serialize)]
pub struct EmbeddingRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
pub struct EmbeddingResponse {
    embedding: Vec<f32>,
}

#[derive(Serialize)]
pub struct LLMRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
struct LLMResponse {
    response: String,
}

#[derive(Debug, Clone)]
pub struct Snippet {
    pub file: String,
    pub content: String,
    pub embedding: Vec<f32>,
}
