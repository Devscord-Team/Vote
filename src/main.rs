pub mod embed_builders;
pub mod event_handlers;

use crate::event_handlers::{message, reaction_add, ready};
use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{Message, Reaction};
use serenity::prelude::*;
use std::env;

rust_i18n::i18n!("locales");

pub struct Handler {
    database: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::handle(&self, ctx, ready).await
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        message::handle(&self, ctx, new_message).await
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        reaction_add::handle(&self, ctx, add_reaction).await
    }
}

#[tokio::main]
async fn main() {
    /* BEFORE START create .env file in root path with content
    DISCORDTOKEN=your_bot_token
    DATABASE_URL=sqlite:watchman-vote.db
    */
    dotenv().ok();

    let handler = create_handler().await;

    let token = env::var("DISCORDTOKEN").unwrap();
    let mut client = Client::builder(
        token,
        GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_PRESENCES
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::GUILD_MESSAGE_REACTIONS,
    )
    .event_handler(handler)
    .await
    .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

async fn create_handler() -> Handler {
    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    Handler { database }
}
