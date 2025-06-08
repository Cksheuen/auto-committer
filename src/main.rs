// main.rs

use reqwest::Client;
use std::error::Error;
use tokio::{self, spawn};
use tokio_stream::StreamExt;

pub mod diff_getter;
pub mod model;

use model::{
    Snippet,
    embedding::{cosine_similarity, fetch_embedding, read_snippet},
};

// === Main ===
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (diff_files_paths, diff_text) = diff_getter::git_diff_in_dir(std::path::Path::new("../pong"))?;
    let client = Client::new();

    let mut raw_snippets = Vec::new();
    for path in diff_files_paths {
        let snippets = read_snippet(&path);
        raw_snippets.extend(snippets);
    }

    let mut indexed_snippets = Vec::new();
    for (file, content) in raw_snippets {
        let embedding = fetch_embedding(&client, &content).await?;
        indexed_snippets.push(Snippet {
            file,
            content,
            embedding,
        });
    }

    let question = "分析代码的更新与变化？
    并给出一条git commit message的建议（要求符合规范）。
    具体规范为：
    1. 格式为：<类型>(<范围>): <描述>
    2. 类型包括：feat（新功能）、fix（修复bug）、docs（文档变更）、style（代码格式变更，不影响功能）、refactor（重构代码，非新增功能或修复bug）、perf（性能优化）、test（增加测试）、chore（其他修改）等。
    3. 描述应简洁明了，使用英文。
    4. 如果有多个变更，使用逗号分隔。
    回答格式为：<git commit message> <换行> <具体分析内容>。";
    let question_embedding = fetch_embedding(&client, question).await?;

    let mut scored: Vec<_> = indexed_snippets
        .iter()
        .map(|s| {
            (
                cosine_similarity(&question_embedding, &s.embedding),
                s.clone(),
            )
        })
        .collect();

    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let top_snippets: Vec<Snippet> = scored.into_iter().take(3).map(|(_, s)| s).collect();

    let answer = model::llm::ask_llm(&client, question, &top_snippets, &diff_text).await?;
    println!("\n💬 回答：\n{}", answer);
    Ok(())
}
