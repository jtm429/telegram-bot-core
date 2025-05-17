use std::env;
use std::fs;

/// Loads a Telegram bot token either from CLI args or a `.token` file.
/// Panics if not provided.
pub fn load_token() -> String {
    let args: Vec<String> = env::args().collect();

    if let Some(pos) = args.iter().position(|x| x == "--token") {
        args.get(pos + 1)
            .expect("Missing value for --token")
            .to_string()
    } else if let Ok(token) = fs::read_to_string(".token") {
        token.trim().to_string()
    } else {
        panic!("Telegram token not provided. Please provide a `.token` file or run with `--token <your_token>`.");
    }
}
