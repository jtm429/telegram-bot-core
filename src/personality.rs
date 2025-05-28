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
- /in <amount> <description>: An amount of income and the description of what it is
- /out <amount> <description>: An amount spent and what it was spent on
- /balance: gets the current balance of the ledger
- /summary: gets a summary with income, expenses, and net.
- /undo: undoes the previous transation.
- /end: ends the bot.
"#;

    let explanation_prompt = format!(
    "You are an assistant that interprets natural language and executes commands on behalf of the user.\n\n\
The user sent the following message:\n\"{}\"\n\n\
You interpreted this message and executed the following command:\n{}\n\n\
Now, explain to the user in plain language what you did and why this command was chosen. \
Speak as though you have already completed the action. ",
    message.trim(),
    command.trim()
);

    let response = self.chatgpt.prompt(&explanation_prompt).await?;

    // Log this interaction to memory
    self.memory.add_user(message);
    self.memory.add_assistant(&response);

    Ok(response)
}

pub async fn interpret_output(
    &mut self,
    user_command: &str,
    command_output: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = format!(
        "You are an assistant that helps users understand the results of their commands.\n\n\
The user ran the following command:\n\"{}\"\n\n\
The command returned the following output:\n{}\n\n\
Now, explain to the user in plain, simple language what this output means. \
If it's a list or data table, summarize what kind of information it contains. \
Avoid repeating the command or output unless necessary — focus on helping the user understand what they're seeing.",
        user_command.trim(),
        command_output.trim()
    );

    let explanation = self.chatgpt.prompt(&prompt).await?;

    // Log the user input and your explanation to memory
    self.memory.add_user(&format!("Command: {}\nOutput:\n{}", user_command, command_output));
    self.memory.add_assistant(&explanation);

    Ok(explanation)
}

pub fn print_bot_mem(&mut self)
{
    println!("Printing memory entries: count = {}", self.memory.return_mem_entries().len());
    for entryy in &self.memory.return_mem_entries()
    {
        println!("{}", entryy)
    }

}

pub async fn add_bot_mem(&mut self, bot_say: &str)
{
    self.memory.add_assistant(bot_say);

    self.print_bot_mem();
}
}
