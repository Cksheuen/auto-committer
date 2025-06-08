use reqwest::Client;
use std::{error::Error, fmt::format};
use std::fs;
use walkdir::WalkDir;
use std::env;

use super::{EmbeddingRequest, EmbeddingResponse};

// === Utilities ===
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot / (norm_a * norm_b + 1e-8)
}

pub fn read_snippet(path: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let content = fs::read_to_string(path).unwrap_or_default();
    for part in content.split("\n\n") {
        if part.trim().len() > 20 {
            results.push((path.to_string(), part.trim().to_string()));
        }
    }
    results
}

pub fn read_snippets(dir: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ["ts", "js", "py", "md", "txt"].contains(&ext.to_str().unwrap()) {
                    let content = fs::read_to_string(path).unwrap_or_default();
                    for part in content.split("\n\n") {
                        if part.trim().len() > 20 {
                            results.push((path.display().to_string(), part.trim().to_string()));
                        }
                    }
                }
            }
        }
    }
    results
}

pub async fn fetch_embedding(client: &Client, text: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    let payload = EmbeddingRequest {
        model: "nomic-embed-text",
        prompt: text,
    };
    let ollama_host = env::var("OLLAMA_SERVER_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string());
    let res = client
        .post(format!("{}/api/embeddings", ollama_host))
        .json(&payload)
        .send()
        .await?
        .json::<EmbeddingResponse>()
        .await?;
    Ok(res.embedding)
}