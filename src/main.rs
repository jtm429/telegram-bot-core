mod bot;
mod token;

use bot::{Bot, IncomingMessage};
use std::time::Duration;
use tokio::time::sleep;

const WAIT_TIME: u64 = 2; // seconds between polling

#[tokio::main]
async fn main() {
    // Load the bot token and create a new bot instance
    let token = token::load_token();
    let mut bot = Bot::new(token);

    println!("Bot is running...");

    // Main loop: poll for messages and respond
    loop {
        if let Some(message) = bot.update().await {
            let reply = handle_message(&message.text);
            bot.send_message(message.chat_id, &reply).await;
        }

        // Wait before checking for the next update
        sleep(Duration::from_secs(WAIT_TIME)).await;
    }
}

// Define how to respond to incoming text messages
fn handle_message(text: &str) -> String {
    text.to_string() // Echo back
}
