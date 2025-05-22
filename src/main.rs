mod bot;
mod token;
mod cgptwrapper;
mod personality;
mod memory;

use bot::{Bot, IncomingMessage, BotControl};
use cgptwrapper::ChatGPTWrapper;
use std::fs;
use std::time::Duration;
use tokio::time::sleep;
use crate::personality::Personality;

const WAIT_TIME: u64 = 2; // seconds between polling

#[tokio::main]
async fn main() {
    let token = token::load_token();
    let mut bot = Bot::new(token).await;

    let raw_prompt = fs::read_to_string("cgpt_nlp_prompt.txt")
        .expect("Failed to read NLP system prompt");
    let gpt = ChatGPTWrapper::new(&raw_prompt); // NLP GPT

    let mut personality = Personality::new(); // Conversational personality

    println!("Bot is running...");

    loop {
        if let Some(message) = bot.update().await {
            if handle_message(&mut bot, &message, &gpt, &mut personality).await {
                break;
            }

            
        }

        sleep(Duration::from_secs(WAIT_TIME)).await;
    }
}

pub async fn handle_message(
    bot: &mut Bot,
    message: &IncomingMessage,
    gpt: &ChatGPTWrapper,
    personality: &mut Personality,
) -> bool {
    let text = &message.text;

    if text.trim() == "/end" {
        bot.end();
        return true;
    }

    // Use NLP model to determine which command to run
    let command = match gpt.prompt(text).await {
        Ok(cmd) => cmd,
        Err(_) => "none".to_string(),
    };
    println!("command: \'{}\'", command.trim());
    if command.trim() != "none" {
    // Log and explain what was done using Personality
    match personality.explain_command(text, &command).await {
        Ok(explanation) => {
            bot.send_message(message.chat_id, &explanation).await;
        }
        Err(err) => {
            eprintln!("Failed to explain command: {}", err);
        }
    }
} else {
    match personality.prompt(&text).await {
        Ok(answer) => {
            bot.send_message(message.chat_id, &answer).await;
        }
        Err(err)=> {
            eprintln!("Failed to answer prompt: {}", err)
        }
    }
}

    if command.trim() == "/end" {
        bot.end();
        return true;
    }

    

    false
}

