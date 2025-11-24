use std::io;

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
    std::process::exit(0);
}