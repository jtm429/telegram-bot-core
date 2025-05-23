mod bot;
mod token;
mod cgptwrapper;
mod personality;
mod memory;
mod accountant;

use bot::{Bot, IncomingMessage, BotControl};
use cgptwrapper::ChatGPTWrapper;
use std::fs;
use std::time::Duration;
use tokio::time::sleep;
use crate::personality::Personality;
use crate::accountant::AccountantBot;

const WAIT_TIME: u64 = 2; // seconds between polling

#[tokio::main]
async fn main() {
    let token = token::load_token();
    let mut bot = Bot::new(token).await;

    let raw_prompt = fs::read_to_string("cgpt_nlp_prompt.txt")
        .expect("Failed to read NLP system prompt");
    let gpt = ChatGPTWrapper::new(&raw_prompt); // NLP GPT

    let mut personality = Personality::new(); // Conversational personality
    let mut accbot = AccountantBot::new();
    println!("Bot is running...");

    loop {
        if let Some(message) = bot.update().await {
            if handle_message(&mut bot, &message, &gpt, &mut personality, &mut accbot).await {
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
    accbot: &mut AccountantBot
) -> bool {
    let text = &message.text;
    println!("{}",text.trim());

    if text.trim() == "/end" {
        bot.end();
        return true;
    }
    if text.trim().starts_with('/')
    {
        
        let mut messybessy = handle_command(accbot, text);
        println!("manual command output: {}", messybessy);
        
        personality.add_bot_mem(&messybessy);
        bot.send_message(message.chat_id, &messybessy);
    } else {


    // Use NLP model to determine which command to run
    let command = match gpt.prompt(text).await {
        Ok(cmd) => cmd,
        Err(_) => "none".to_string(),
    };
    println!("command: \'{}\'", command.trim());
    if command.trim() != "none" {
        let response = handle_command(accbot, &command);

    
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

}

    false
}
pub fn handle_command(bot: &mut AccountantBot, input: &str) -> String {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return "No command provided.".to_string();
    }

    match parts[0] {
        "/income" | "/in" if parts.len() >= 3 => {
            if let Ok(amount) = parts[1].parse::<f64>() {
                let desc = parts[2..].join(" ");
                bot.income(amount, &desc);
                format!("Logged income: {:.2} - {}", amount, desc)
            } else {
                "Invalid amount.".to_string()
            }
        }
        "/out" if parts.len() >= 3 => {
            if let Ok(amount) = parts[1].parse::<f64>() {
                let desc = parts[2..].join(" ");
                bot.out(amount, &desc);
                format!("Logged expense: {:.2} - {}", amount, desc)
            } else {
                "Invalid amount.".to_string()
            }
        }
        "/balance" => format!("Current balance: {:.2}", bot.balance()),

        "/summary" => {
            let (inc, exp, net) = bot.summary();
            format!("Income: {:.2}, Expenses: {:.2}, Net: {:.2}", inc, exp, net)
        }

        "/recent" if parts.len() == 2 => {
            if let Ok(n) = parts[1].parse::<usize>() {
                bot.list_recent(n)
            } else {
                "Invalid number.".to_string()
            }
        }

        "/undo" => match bot.undo() {
            Some(entry) => format!("Undid last entry: {:.2} - {}", entry.amount, entry.description),
            None => "Nothing to undo.".to_string(),
        },

        _ => "Unknown command. Try /in, /out, /balance, /summary, /recent, or /undo.".to_string(),
    }
}

