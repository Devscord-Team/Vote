use dotenv::dotenv;
use rust_i18n::t;
use serenity::async_trait;
use serenity::builder::CreateEmbedAuthor;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{Message, ReactionType};
use serenity::model::user::User;
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
        let create_vote_channel_id: u64 = 636638300731604996;
        if new_message.channel_id.as_u64() != &create_vote_channel_id {
            return;
        }
        let sent_message_id = new_message
            .channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.author(|a| map_author_to_embed(a, new_message.author))
                        .title(t!("vote_asked.title", locale = "pl", approve_emoji = "✅"))
                        .description(&new_message.content)
                })
            })
            .await
            .expect("Error sending message")
            .id;

        new_message
            .channel_id
            .create_reaction(
                &ctx.http,
                sent_message_id,
                ReactionType::Unicode("✅".to_string()),
            )
            .await
            .expect("Error adding reaction to message");
    }
}

fn map_author_to_embed(builder: &mut CreateEmbedAuthor, author: User) -> &mut CreateEmbedAuthor {
    builder
        .name(format!("{}#{}", &author.name, &author.discriminator))
        .icon_url(match &author.avatar_url() {
            Some(x) => x.to_owned(),
            None => author.default_avatar_url(),
        })
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
