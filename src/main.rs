use dotenv::dotenv;
use rust_i18n::t;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Message;
use serenity::prelude::*;
use std::env;

rust_i18n::i18n!("locales");

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        if new_message.author.bot {
            return;
        }
        new_message
            .channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(t!("hello", locale = "pl"))
                        .description("description")
                })
            })
            .await
            .expect("Error sending message");
    }
}

#[tokio::main]
async fn main() {
    /* BEFORE START create .env file in root path with content
    DISCORDTOKEN=your_bot_token
    */
    dotenv().ok();

    let token = env::var("DISCORDTOKEN").unwrap();
    let mut client = Client::builder(
        token,
        GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_PRESENCES
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::GUILD_MESSAGE_REACTIONS,
    )
    .event_handler(Handler)
    .await
    .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
