use std::env;
extern crate dotenv;

use dotenv::dotenv;

use gemini_rust::prelude::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::fs::read_to_string;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content.starts_with("hey vipseek") {
            // first reply that vip is typing
            let response = MessageBuilder::new().push("vip is typing...").build();

            if let Err(why) = msg.reply(&context.http, response).await {
                println!("Error sending msg: {why:?}");
            }

            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {why:?}");
                    return;
                }
            };

            let system_prompt = read_to_string("SYSTEM_PROMPT.md");

            let gemini_key = env::var("GEMINI_API_KEY").expect("No Gemini API key provided!");

            let gemini_client = Gemini::new(gemini_key).expect("Failed to create Gemini client");
            println!("Gemini client created?");

            let channel_name = channel.id().name(&context.http).await;
            let user_name = &msg.author.name;

            let msg_content = &msg.content;

            println!("{user_name:?}, channel {channel_name:?}: {msg_content:?}");

            let ai_response = gemini_client
                .create_interaction()
                .with_model("gemini-3.8-flash")
                .with_text(format!(
                    "{system_prompt:?}\n\n Channel name: {channel_name:?}\nUser name: {user_name:?}\n, Message: {msg_content:?}\n",
                ))
                .execute()
                .await.expect("failed to get Gemini API response");

            if let Err(why) = msg.reply(&context.http, ai_response.output_text()).await {
                println!("Error sending msg: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}!", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("No token provided!");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}")
    }
}
