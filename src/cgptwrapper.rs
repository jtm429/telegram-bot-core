use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug)]
pub struct ChatGPTWrapper {
    api_key: String,
    system_prompt: String,
    client: Client,
}

#[derive(Serialize)]
struct ChatGPTRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatGPTResponse {
    choices: Vec<ChatGPTChoice>,
}

#[derive(Deserialize)]
struct ChatGPTChoice {
    message: ChatMessageOwned,
}

#[derive(Deserialize)]
struct ChatMessageOwned {
    role: String,
    content: String,
}

impl ChatGPTWrapper {
    pub fn new(system_prompt: &str) -> Self {
        let api_key = fs::read_to_string("cgpt.token")
            .expect("Failed to read cgpt.token")
            .trim()
            .to_string();

        ChatGPTWrapper {
            api_key,
            system_prompt: system_prompt.to_string(),
            client: Client::new(),
        }
    }

    pub async fn prompt(&self, user_input: &str) -> Result<String, reqwest::Error> {
        let request_body = ChatGPTRequest {
            model: "gpt-4o",
            messages: vec![
                ChatMessage {
                    role: "system",
                    content: &self.system_prompt,
                },
                ChatMessage {
                    role: "user",
                    content: user_input,
                },
            ],
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()
            .await?;

        let result: ChatGPTResponse = response.json().await?;
        Ok(result.choices[0].message.content.trim().to_string())
    }
}
