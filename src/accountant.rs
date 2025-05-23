use chrono::NaiveDate;
use std::collections::VecDeque;
use std::io::Write;

#[derive(Debug, Clone)]
pub enum EntryType {
    Income,
    Expense,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub entry_type: EntryType,
    pub amount: f64,
    pub description: String,
    pub date: NaiveDate,
    pub category: Option<String>,
}

pub struct AccountantBot {
    pub entries: VecDeque<Entry>,
}

impl AccountantBot {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
        }
    }

    pub fn income(&mut self, amount: f64, description: &str) {
        self.entries.push_back(Entry {
            entry_type: EntryType::Income,
            amount,
            description: description.to_string(),
            date: chrono::Local::now().naive_local().date(),
            category: None,
        });
    }

    pub fn out(&mut self, amount: f64, description: &str) {
        self.entries.push_back(Entry {
            entry_type: EntryType::Expense,
            amount,
            description: description.to_string(),
            date: chrono::Local::now().naive_local().date(),
            category: None,
        });
    }

    pub fn balance(&self) -> f64 {
        self.entries.iter().map(|e| match e.entry_type {
            EntryType::Income => e.amount,
            EntryType::Expense => -e.amount,
        }).sum()
    }

    pub fn undo(&mut self) -> Option<Entry> {
        self.entries.pop_back()
    }

    pub fn summary(&self) -> (f64, f64, f64) {
        let income: f64 = self.entries.iter()
            .filter(|e| matches!(e.entry_type, EntryType::Income))
            .map(|e| e.amount)
            .sum();
        let expense: f64 = self.entries.iter()
            .filter(|e| matches!(e.entry_type, EntryType::Expense))
            .map(|e| e.amount)
            .sum();
        let net = income - expense;
        (income, expense, net)
    }

    pub fn list_recent(&self, count: usize) -> String {
        let mut result = String::new();
        for entry in self.entries.iter().rev().take(count) {
            let kind = match entry.entry_type {
                EntryType::Income => "IN",
                EntryType::Expense => "OUT",
            };
            let category = entry.category.as_deref().unwrap_or("uncategorized");
            result.push_str(&format!(
                "{} | {} | {:.2} | {}\n",
                kind,
                entry.date,
                entry.amount,
                format!("{}: {}", category, entry.description)
            ));
        }
        result
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        for entry in &self.entries {
            let kind = match entry.entry_type {
                EntryType::Income => "IN",
                EntryType::Expense => "OUT",
            };
            let category = entry.category.clone().unwrap_or_else(|| "uncategorized".to_string());
            writeln!(
                file,
                "{},{},{},{},{}",
                kind, entry.date, entry.amount, category, entry.description.replace(",", " ")
            )?;
        }
        Ok(())
    }

    pub fn load_from_file(&mut self, path: &str) -> std::io::Result<()> {
        let content = std::fs::read_to_string(path)?;
        self.entries.clear();
        for line in content.lines() {
            let parts: Vec<&str> = line.splitn(5, ',').collect();
            if parts.len() != 5 { continue; }
            let entry_type = match parts[0] {
                "IN" => EntryType::Income,
                "OUT" => EntryType::Expense,
                _ => continue,
            };
            let date = parts[1]
                .parse::<NaiveDate>()
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid date"))?;

            let amount = parts[2]
                .parse::<f64>()
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid amount"))?;
            let category = Some(parts[3].to_string());
            let description = parts[4].to_string();
            self.entries.push_back(Entry {
                entry_type,
                amount,
                description,
                date,
                category,
            });
        }
        Ok(())
    }
}
