use std::env;
extern crate dotenv;

use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::fs::read_to_string;
use std::sync::LazyLock;

use groq_api_rs::completion::{
    client::CompletionOption, client::Groq, message::Message as GroqMessage, request::builder,
};

static GROQ_API_KEY: LazyLock<String> =
    LazyLock::new(|| env::var("GROQ_API_KEY").expect("No Groq API key provided!"));

static GROQ_MODEL: LazyLock<String> =
    LazyLock::new(|| env::var("GROQ_MODEL").expect("Choose a groq model!"));

static SYSTEM_PROMPT: LazyLock<String> =
    LazyLock::new(|| read_to_string("SYSTEM_PROMPT.md").expect("Where's the system prompt?"));

struct Handler;

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
                    println!("Error getting channel: {why:?}");
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
            let system_prompt: Vec<&str> = LazyLock::force(&SYSTEM_PROMPT).split("---").collect();

            // rudimentary logging
            println!("{user_name:?}, channel {channel_name:?}: {msg_content:?}");

            let groq_msg = vec![
                GroqMessage::SystemMessage {
                    role: Some("system".to_string()),
                    content: Some(system_prompt[0].to_string()),
                    name: None,
                    tool_call_id: None,
                },
                GroqMessage::UserMessage {
                    role: Some("user".to_string()),
                    content: Some(
                        format!("{}\nUser name: {user_name:?}, Display name: {display_name:?}, Channel: {channel_name:?}, Message: {msg_content:?}", system_prompt[1]).to_string(),
                    ),
                    name: None,
                    tool_call_id: None,
                },
            ];
            // call the Gemini API while constructing the full prompt on the fly
            let mut groq_client = Groq::new(&GROQ_API_KEY);
            let request = builder::RequestBuilder::new(GROQ_MODEL.to_string());
            groq_client.add_messages(groq_msg);
            let ai_response = groq_client.create(request).await;

            match ai_response {
                Ok(CompletionOption::NonStream(v)) => {
                    // cut 1999 characters because Discord
                    let final_response: String = v
                        .choices
                        .first()
                        .map(|choice| choice.message.content.clone())
                        .unwrap_or_default()
                        .chars()
                        .take(1999)
                        .collect();

                    // send the AI reply.
                    // That's it. that's the bot. I made the thing in like a day or something. While being new to Rust.
                    if let Err(why) = msg.reply(&context.http, final_response).await {
                        println!("Error sending msg: {why:?}")
                    }
                }
                Ok(CompletionOption::Stream(_)) => {
                    // This block intentionally left blank
                }
                Err(e) => {
                    println!("Failed to get Groq API response: {e:?}");
                    if let Err(why) = msg
                        .reply(
                            &context.http,
                            "Giờ vip-chan hơi mệt, đợi tí rồi kêu lại nya~",
                        )
                        .await
                    {
                        println!("Error sending error msg: {why:?}")
                    }
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
    dotenv().ok();
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
        println!("Client error: {why:?}")
    }
}
