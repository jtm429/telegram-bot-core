
# Telegram Bot Template (Rust)

This project is a template to be used when making Telegram bots.

Gone are the days when I will get a bot idea and spend hours fiddling with the API when there's code to be written.

---

## 📜 Hark, Yee Traveler

Should thou wish to fork my content, thou should be aware of several things:

1. **An API key** must be created using the Telegram [@BotFather](https://t.me/botfather).
2. This key should be saved in a file called **`.token`** (do *not* commit this file).
3. Additionally, thou shall need to create a file named **`allowed_users.txt`** containing **your Telegram user ID**.
4. Thou must now also prepare the following scrolls of personality and purpose:
    - **`cgpt.token`**: An OpenAI API token, for summoning language models.
    - **`cgpt_nlp_prompt.txt`**: The sacred prompt that tells GPT how to interpret user messages. You will need to edit it to allow GPT to execute the codes.
    - **`personality.txt`**: A character-defining missive that bestows sass, angst, or professionalism upon the bot's soul.

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

---

## 🎭 On the Matter of Magical Buttons and Their Responses

In the pursuit of grander user experience and the folly of human laziness, thou mayst now summon messages bearing **enchanted buttons**—symbols of decision, gates of action, vessels of interaction.

### 📩 `send_message_with_buttons(chat_id, text, options)`

With this sacred incantation, thou may conjure a message with clickable runes (buttons) beneath it.

These buttons, once pressed, shall whisper their **label** back to thee via the callback winds of Telegram.

```rust
bot.send_message_with_buttons(
    chat_id,
    "Choose thy path, adventurer:",
    vec!["Enter the Cave", "Return to Town"]
).await;
```

No spellbook knowledge is required of the user—they must merely tap.

---

### ⏳ `await_callback_once()`

Once the runes are summoned, thou may invoke this ritual to **pause** and **listen** for a single button-press from the stars above.

```rust
if let Some((chat_id, choice)) = bot.await_callback_once().await {
    println!("The adventurer chose: {}", choice);
    // Now act upon their chosen fate...
}
```

Beware: this awaits only **one** such sign before completing. 'Tis not meant for long vigils.

---

### 🧪 Practical Alchemy

When combined, these functions allow for **structured choices** and **interactive journeys**:

```rust
bot.send_message_with_buttons(chat_id, "Pick a relic to examine:", vec!["Orb", "Amulet", "Blade"]).await;

if let Some((_chat_id, artifact)) = bot.await_callback_once().await {
    println!("You gaze into the {}", artifact);
}
```

Thus, your bot shall gain the power to **ask questions** and **react to answers**, like a courteous host in a realm of automata.

---

May your bots run smoothly, your GPT prompts be clever, and your system personalities delightfully dysfunctional.
