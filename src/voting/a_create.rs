use rust_i18n::t;
use serenity::{
    model::prelude::{Message, MessageId, ReactionType},
    prelude::Context,
};

use crate::{embed_builders, Handler};

pub async fn handle(handler: &Handler, ctx: Context, new_message: Message) {
    let sent_message_id = new_message
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.author(|a| embed_builders::author::build(a, new_message.author.clone()))
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

    insert_to_db(handler, new_message, sent_message_id).await;
}

async fn insert_to_db(handler: &Handler, new_message: Message, sent_message_id: MessageId) {
    let author = new_message.author;
    let author_name = format!("{}#{}", author.name, author.discriminator);
    let author_id = i64::try_from(author.id.as_u64().to_owned()).unwrap();
    let server_id = i64::try_from(new_message.guild_id.unwrap().as_u64().to_owned()).unwrap();
    let message_id = i64::try_from(sent_message_id.as_u64().to_owned()).unwrap();

    let result = sqlx::query!(
        "INSERT INTO Votes 
            (Author, AuthorID, Content, ServerID, MessageID,VerificationMessageID, IsApprovedByAuthor, ApprovedByAdminId) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        author_name,
        author_id,
        new_message.content,
        server_id,
        message_id,
        0,
        0,
        0
    )
    .execute(&handler.database)
    .await;

    match result {
        Ok(x) => println!("{}", x.last_insert_rowid()),
        Err(x) => println!("{}", x),
    }
}
