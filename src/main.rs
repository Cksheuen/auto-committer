use std::error::Error;
use std::path::PathBuf;
use std::{env, fs, path::Path, time::Duration};
use tokio;

use notify_debouncer_full::{DebounceEventResult, new_debouncer, notify::*};

pub mod diff_getter;
pub mod model;

// === Main ===
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let environment = env::var("OLLAMA_SERVER_HOST").unwrap_or_else(|_| "development".to_string());
    println!("当前环境: {}", environment);

    let path = Path::new("./");

    let git_ignore_content = fs::read_to_string(path.join(".gitignore")).unwrap();
    let gitignore = git_ignore_content
        .lines()
        .map(|x| x.to_owned())
        .collect::<Vec<String>>();

    let mut debouncer = new_debouncer(
        Duration::from_secs(2),
        None,
        move |result: DebounceEventResult| match result {
            Ok(events) => events.iter().for_each(|event| {
                let paths = event
                    .paths
                    .iter()
                    .filter(|path| {
                        let path_str = path
                            .as_path()
                            .iter()
                            .map(|x| x.to_str().unwrap().to_owned())
                            .collect::<Vec<String>>();

                        return path_str.iter().all(|x| !gitignore.contains(x));
                    })
                    .collect::<Vec<&PathBuf>>();
                if paths.len() > 0 {
                    println!("changed path: {:?}", paths);
                }
            }),
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        },
    )
    .unwrap();

    debouncer.watch(".", RecursiveMode::Recursive).unwrap();

    loop {}
}
