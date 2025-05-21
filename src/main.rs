mod bot;
mod token;

use bot::{Bot, IncomingMessage, BotControl};
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
            if handle_message(&mut bot, &message.text) {
                break;
            }
            bot.send_message(message.chat_id, &message.text).await;
        }

        // Wait before checking for the next update
        sleep(Duration::from_secs(WAIT_TIME)).await;
    }
}

// Return true to end bot
fn handle_message(bot: &mut bot::Bot, text: &str) -> bool {
    if text.trim() == "/end" {
        bot.end();
        return true;
    }
    false
}

