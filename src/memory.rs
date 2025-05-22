pub enum Role {
    User,
    Assistant,
}

pub struct Entry {
    pub role: Role,
    pub content: String,
}

pub struct Memory {
    entries: Vec<Entry>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            entries: Vec::new(),
        }
    }

    pub fn add_user(&mut self, content: &str) {
        self.entries.push(Entry {
            role: Role::User,
            content: content.to_string(),
        });
    }

    pub fn add_assistant(&mut self, content: &str) {
        self.entries.push(Entry {
            role: Role::Assistant,
            content: content.to_string(),
        });
    }

    pub fn recent_context(&self, prompt: &str) -> String {
        let start = if self.entries.len() > 5 {
            self.entries.len() - 5
        } else {
            0
        };

        let mut context = String::new();
        for entry in &self.entries[start..] {
            let role = match entry.role {
                Role::User => "User",
                Role::Assistant => "Assistant",
            };
            context.push_str(&format!("{}: {}\n", role, entry.content));
        }

        // Append current prompt as if the user just sent it
        context.push_str(&format!("User: {}\nAssistant:", prompt));

        context
    }
}
