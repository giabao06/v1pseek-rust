use std::env;
extern crate dotenv;

use dotenv::dotenv;

use gemini_rust::prelude::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::fs::read_to_string;
use std::sync::LazyLock;

const DISCORD_MESSAGE_LIMIT: usize = 2000;
const GENERIC_USER_ERROR: &str = "Giờ vip-chan hơi mệt, đợi tí rồi kêu lại nya~";

static GEMINI_API_KEY: LazyLock<String> =
    LazyLock::new(|| env::var("GEMINI_API_KEY").expect("No Gemini API key provided!"));

static GEMINI_MODEL: LazyLock<String> =
    LazyLock::new(|| env::var("GEMINI_MODEL").expect("Choose a gemini model!"));

static SYSTEM_PROMPT: LazyLock<String> =
    LazyLock::new(|| read_to_string("SYSTEM_PROMPT.md").expect("Where's the system prompt?"));

static GEMINI_CLIENT: LazyLock<Gemini> = LazyLock::new(|| {
    Gemini::new(LazyLock::force(&GEMINI_API_KEY)).expect("Failed to create client!")
});

struct Handler;

fn split_response_chunks(text: &str, max_chars: usize) -> Vec<String> {
    if text.is_empty() || max_chars == 0 {
        return Vec::new();
    }

    text.chars()
        .collect::<Vec<char>>()
        .chunks(max_chars)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

async fn notify_user_error(context: &Context, msg: &Message, text: &str) {
    if let Err(why) = msg.channel_id.say(&context.http, text).await {
        eprintln!("Error sending fallback error msg: {why:?}");
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        // Trigger only when these words are in the start of the message
        if ["hey vip", "hey vip-chan", "hey vipseek"]
            .iter()
            .any(|s| msg.content.to_lowercase().starts_with(*s))
            && !msg.author.bot
        {
            // Figure out which channel the message is sent to
            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(why) => {
                    eprintln!("Error getting channel: {why:?}");
                    notify_user_error(&context, &msg, GENERIC_USER_ERROR).await;
                    return;
                }
            };

            // Send the Typing status ("Vip is typing...")
            let _typing = channel.id().start_typing(&context.http);

            // Gather info
            let channel_name = channel.id().name(&context.http).await;
            let user_name = &msg.author.name;
            let display_name = &msg.author.display_name();
            let msg_content = &msg.content_safe(&context.cache);
            let Some((system_instruction, prompt_prefix)) = LazyLock::force(&SYSTEM_PROMPT).split_once("---") else {
                eprintln!("Invalid system prompt format: expected delimiter '---'");
                notify_user_error(&context, &msg, GENERIC_USER_ERROR).await;
                return;
            };

            // rudimentary logging
            println!("{user_name:?}, channel {channel_name:?}: {msg_content:?}");

            // call the Gemini API while constructing the full prompt on the fly
            let ai_response = GEMINI_CLIENT
                .create_interaction()
                .with_model(GEMINI_MODEL.clone())
                .with_system_instruction(system_instruction.to_string())
                .with_text(format!(
                    "{}\nChannel name: {channel_name:?}\nUser name: {user_name:?}\n, Display name: {display_name:?}\n Message: {msg_content:?}\n", prompt_prefix
                ))
                .execute()
                .await;

            match ai_response {
                Ok(v) => {
                    let response_chunks =
                        split_response_chunks(&v.output_text(), DISCORD_MESSAGE_LIMIT);

                    if response_chunks.is_empty() {
                        notify_user_error(&context, &msg, GENERIC_USER_ERROR).await;
                        return;
                    }

                    let mut chunks = response_chunks.into_iter();

                    if let Some(first_chunk) = chunks.next()
                        && let Err(why) = msg.reply(&context.http, first_chunk).await
                    {
                        eprintln!("Error sending first response chunk: {why:?}");
                        notify_user_error(&context, &msg, GENERIC_USER_ERROR).await;
                        return;
                    }

                    for chunk in chunks {
                        if let Err(why) = msg.channel_id.say(&context.http, chunk).await {
                            eprintln!("Error sending continued response chunk: {why:?}");
                            notify_user_error(&context, &msg, GENERIC_USER_ERROR).await;
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to get Gemini API response: {e:?}");
                    notify_user_error(&context, &msg, GENERIC_USER_ERROR).await;
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}!", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    // load dotenv
    if let Err(why) = dotenv() {
        eprintln!("Warning: failed loading .env file: {why}");
    }
    // take the token and set intenst
    // We just need to read Server messages and message contents
    let token = env::var("BOT_TOKEN").expect("No token provided!");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // construct the client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");
    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::{DISCORD_MESSAGE_LIMIT, split_response_chunks};

    #[test]
    fn keeps_short_response_in_single_chunk() {
        let chunks = split_response_chunks("hello", DISCORD_MESSAGE_LIMIT);
        assert_eq!(chunks, vec!["hello"]);
    }

    #[test]
    fn splits_long_response_into_multiple_chunks() {
        let text = "a".repeat(DISCORD_MESSAGE_LIMIT * 2 + 1);
        let chunks = split_response_chunks(&text, DISCORD_MESSAGE_LIMIT);

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].chars().count(), DISCORD_MESSAGE_LIMIT);
        assert_eq!(chunks[1].chars().count(), DISCORD_MESSAGE_LIMIT);
        assert_eq!(chunks[2].chars().count(), 1);
    }
}
