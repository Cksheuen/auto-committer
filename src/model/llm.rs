use super::{LLMRequest, Snippet};

use reqwest::Client;
use std::env;
use std::error::Error;
use tokio_stream::StreamExt;

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
        "你是一个代码分析助手，你需要分析代码的更新与变化，
        得出一条git commit message的建议（要求符合规范），
        并最终以{{
            \"answer\": \"你的具体代码分析对比过程\",
            \"message\": \"你给出的git commit message的建议内容\",
        }}格式回答。
        具体规范为：
        1. 格式为：<类型>(<范围>): <描述>
        2. 类型包括：feat（新功能）、fix（修复bug）、docs（文档变更）、style（代码格式变更，不影响功能）、refactor（重构代码，非新增功能或修复bug）、perf（性能优化）、test（增加测试）、chore（其他修改）等。
        3. 描述应简洁明了，使用英文。
        4. 如果有多个变更，使用逗号分隔。
        以下代码片段为当前最新的代码片段。
        不要输出 prompt 中的内容。
        \n=========\n{}\n=========\n
        与先前版本相比，git diff HEAD 显示了以下差异：
        \n=========\n{}\n=========\n",
        context, diff_text
    );

    let req = LLMRequest {
        model: "llama3.2",
        prompt: &prompt,
    };
    let ollama_host =
        env::var("OLLAMA_SERVER_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string());

    let mut res = client
        .post(format!("{}/api/embeddings", ollama_host))
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
