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
            let system_prompt = LazyLock::force(&SYSTEM_PROMPT);

            // rudimentary logging
            println!("{user_name:?}, channel {channel_name:?}: {msg_content:?}");

            // call the Gemini API while constructing the full prompt on the fly
            let ai_response = GEMINI_CLIENT
                .create_interaction()
                .with_model(GEMINI_MODEL.clone())
                .with_text(format!(
                    "{system_prompt:?}\n\n Channel name: {channel_name:?}\nUser name: {user_name:?}\n, Display name: {display_name:?}\n Message: {msg_content:?}\n",
                ))
                .execute()
                .await.expect("failed to get Gemini API response");

            // cut 1999 characters because Discord
            let final_response: String = ai_response.output_text().chars().take(1999).collect();

            // send the AI reply.
            // That's it. that's the bot. I made the thing in like a day or something. While being new to Rust.
            if let Err(why) = msg.reply(&context.http, final_response).await {
                println!("Error sending msg: {why:?}")
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
