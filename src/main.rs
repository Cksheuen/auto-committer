use std::error::Error;
use tokio;
use std::env;

pub mod diff_getter;
pub mod model;

// === Main ===
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let environment = env::var("OLLAMA_SERVER_HOST").unwrap_or_else(|_| "development".to_string());
    println!("当前环境: {}", environment);
    let answer = model::compare("./").await?;
    println!("\n💬 回答：\n{}", answer);
    Ok(())
}
