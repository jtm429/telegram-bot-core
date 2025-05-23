use std::fs;
use crate::cgptwrapper::ChatGPTWrapper;
use crate::memory::{Memory, Role};

pub struct Personality {
    chatgpt: ChatGPTWrapper,
    memory: Memory,
}

impl Personality {
    pub fn new() -> Self {
        let prompt = fs::read_to_string("personality.txt")
            .expect("Failed to read personality.txt")
            .trim()
            .to_string();

        let chatgpt = ChatGPTWrapper::new(&prompt);
        let memory = Memory::new();

        Personality { chatgpt, memory }
    }

pub async fn prompt(&mut self, user_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let full_prompt = self.memory.recent_context(user_input);
    let response = self.chatgpt.prompt(&full_prompt).await?;
    self.memory.add_user(user_input);
    self.memory.add_assistant(&response);
    Ok(response)
}

pub async fn explain_command(&mut self, message: &str, command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let command_list = r#"
Available commands:
- /end: ends the bot.
"#;

    let explanation_prompt = format!(
    "You are an assistant that interprets natural language and executes commands on behalf of the user.\n\n\
The user sent the following message:\n\"{}\"\n\n\
You interpreted this message and executed the following command:\n{}\n\n\
Now, explain to the user in plain language what you did and why this command was chosen. \
Speak as though you have already completed the action.",
    message.trim(),
    command.trim()
);

    let response = self.chatgpt.prompt(&explanation_prompt).await?;

    // Log this interaction to memory
    self.memory.add_user(message);
    self.memory.add_assistant(&response);

    Ok(response)
}

pub async fn add_bot_mem(&mut self, bot_say: &str)
{
    self.memory.add_assistant(bot_say);
    println!(self.memory.return_mem_entries())
}
}
