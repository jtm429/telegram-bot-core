# Telegram Bot Template (Rust)

This project is a template to be used when making Telegram bots.

Gone are the days when I will get a bot idea and spend hours fiddling with the API when there's code to be written.

---

## 📜 Hark, Yee Traveler

Should thou wish to fork my content, thou should be aware of several things:

1. **An API key** must be created using the Telegram [@BotFather](https://t.me/botfather).
2. This key should be saved in a file called **`.token`** (do *not* commit this file).
3. Additionally, thou shall need to create a file named **`allowed_users.txt`** containing **your Telegram user ID**.

Example:
```
123456789
```

You can get your Telegram ID by messaging [@userinfobot](https://t.me/userinfobot).

---

## ⚔️ Access Control

The bots I intend to create are for **personal use only**.

If thou should wish to create a **public-facing bot**, thou must:
- Navigate to `src/bot.rs`
- Comment out or remove the lines that check the user's ID against `allowed_users.txt`

---

## 🚀 Running the Bot

```bash
cargo build --release
cargo run --release
```

Or run the built binary:

```bash
./target/release/telegram-bot-core
```

May your bots run smoothly and your updates never be duplicated.
