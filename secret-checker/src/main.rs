use std::io;
use std::env;
use std::path::PathBuf;

use secret_checker::PreToolUseInput;


fn main() {
    let input: PreToolUseInput = match serde_json::from_reader(io::stdin()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            std::process::exit(2);
        }
    };

    if input.tool_name().to_lowercase() != "bash" {
        dbg!("tool name not bash: tool_name={:?}", input.tool_name());
        std::process::exit(0);
    }

    let command = match input.tool_input().content() {
        Some(content) => content,
        None => {
            dbg!("no content: tool_input={:?}", input.tool_input());
            std::process::exit(0);
        }
    };
    dbg!("command={:?}", command);
    if !command.contains("git commit") {
        dbg!("command does not contain git commit");
        std::process::exit(0);
    }
    let project_dir = env::var("CLAUDE_PROJECT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::current_dir()
        .expect("Failed to get current directory"));
    dbg!("project_dir={:?}", project_dir);
    std::process::exit(0);
}