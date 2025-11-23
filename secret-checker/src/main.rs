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
        dbg!("{:?}", input.tool_name());
        std::process::exit(0);
    }

    let command = input.tool_input().content();
    dbg!("{:?}", command);
    std::process::exit(0);
}