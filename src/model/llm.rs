use super::{Snippet, LLMRequest};

use tokio_stream::StreamExt;
use reqwest::Client;
use std::error::Error;

pub async fn ask_llm(
    client: &Client,
    question: &str,
    snippets: &[Snippet],
    diff_text: &str,
) -> Result<String, Box<dyn Error>> {
    let context = snippets
        .iter()
        .map(|s| format!("// File: {}\n{}", s.file, s.content))
        .collect::<Vec<_>>()
        .join("\n\n");

    let prompt = format!(
        "你是一个代码分析助手，需要根据用户的提问严格按照给出的回答模版回答。以下代码片段为当前最新的代码片段。
        \n=========\n{}\n=========\n
        与先前版本相比，git diff HEAD 显示了以下差异：
        \n=========\n{}\n=========\n
        {}\n",
        context, diff_text, question
    );

    let req = LLMRequest {
        model: "llama3.2",
        prompt: &prompt,
    };

    let mut res = client
        .post("http://localhost:11434/api/generate")
        .json(&req)
        .send()
        .await?
        .bytes_stream();

    let mut full_response = String::new();

    while let Some(item) = res.next().await {
        let chunk = item?; // 先解 Result，? 操作符会返回错误
        let chunk_str = std::str::from_utf8(&chunk)?;
        for line in chunk_str.lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(response) = json.get("response").and_then(|r| r.as_str()) {
                    full_response.push_str(response);
                }
            }
        }
    }

    Ok(full_response)
}
