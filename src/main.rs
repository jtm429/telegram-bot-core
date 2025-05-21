mod bot;
mod token;
mod cgptwrapper;

use bot::{Bot, IncomingMessage, BotControl};
use cgptwrapper::ChatGPTWrapper;
use std::fs;
use std::time::Duration;
use tokio::time::sleep;

const WAIT_TIME: u64 = 2; // seconds between polling

#[tokio::main]
async fn main() {
    // Load the bot token and create a new bot instance
    let token = token::load_token();
    let mut bot = Bot::new(token);

    // Load the GPT system prompt from file
    let raw_prompt = fs::read_to_string("cgpt_nlp_prompt.txt")
        .expect("Failed to read NLP system prompt");
    let gpt = ChatGPTWrapper::new(&raw_prompt);

    println!("Bot is running...");

    // Main loop: poll for messages and respond
    loop {
        if let Some(message) = bot.update().await {
            if handle_message(&mut bot, &message.text, &gpt).await {
                break;
            }
            bot.send_message(message.chat_id, &message.text).await;
        }

        // Wait before checking for the next update
        sleep(Duration::from_secs(WAIT_TIME)).await;
    }
}

// Return true to end bot
async fn handle_message(bot: &mut bot::Bot, text: &str, gpt: &ChatGPTWrapper) -> bool {
    // Fast path for direct command
    if text.trim() == "/end" {
        bot.end();
        return true;
    }

    // Otherwise, ask GPT
    let command = gpt.prompt(text).await.unwrap_or_else(|_| "None".to_string());

    if command.trim() == "/end" {
        bot.end();
        return true;
    }

    false
}

