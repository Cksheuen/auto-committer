use std::process::Command;
use std::path::Path;
use std::error::Error;

pub fn git_diff_in_dir(dir: &Path) -> Result<(Vec<String>, String), Box<dyn Error>> {
    let output = Command::new("git")
        .args(&["diff", "HEAD"])
        .current_dir(dir)
        .output()?;  // 执行命令并捕获输出

    if !output.status.success() {
        // 如果git diff命令执行失败，可以返回错误
        return Err(format!("git diff failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    // 转换 stdout 为 UTF-8 字符串
    let diff_text = String::from_utf8(output.stdout)?;
    
    let lines: Vec<&str> = diff_text.lines().collect();
    let filtered_lines: Vec<String> = lines
    .into_iter()
    .filter(|line| line.starts_with("diff --git"))
    .map(|line| 
        line.to_string()
            .replace("diff --git", "")
            .trim()
            .split(' ').collect::<Vec<&str>>()[0]
            .replace("a/", 
            format!("{}/",dir.to_str().unwrap().to_string()).as_str())
    )
    .collect();

    Ok((filtered_lines, diff_text))
}