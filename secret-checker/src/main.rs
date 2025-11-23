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

    let command = input.tool_input().content();
    //println!("{:?}", input);
    std::process::exit(0);
}