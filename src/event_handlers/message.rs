use rust_i18n::t;
use serenity::builder::CreateEmbedAuthor;
use serenity::model::prelude::{Message, ReactionType};
use serenity::model::user::User;
use serenity::prelude::*;

use crate::Handler;

pub async fn handle(handler: &Handler, ctx: Context, new_message: Message) {
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
                e.author(|a| map_author_to_embed(a, new_message.author.clone()))
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

    let author = new_message.author;
    let author_name = format!("{}#{}", author.name, author.discriminator);
    let author_id = i64::try_from(author.id.as_u64().to_owned()).unwrap();
    let server_id = i64::try_from(new_message.guild_id.unwrap().as_u64().to_owned()).unwrap();
    let message_id = i64::try_from(sent_message_id.as_u64().to_owned()).unwrap();

    let result = sqlx::query!(
        "INSERT INTO Votes 
            (Author, AuthorID, Content, ServerID, MessageID, IsApprovedByAuthor, ApprovedByAdminId) 
            VALUES (?, ?, ?, ?, ?, ?, ?)",
        author_name,
        author_id,
        new_message.content,
        server_id,
        message_id,
        0,
        0
    )
    .execute(&handler.database) // < Where the command will be executed
    .await;

    match result {
        Ok(x) => println!("{}", x.last_insert_rowid()),
        Err(x) => println!("{}", x),
    }
}

fn map_author_to_embed(builder: &mut CreateEmbedAuthor, author: User) -> &mut CreateEmbedAuthor {
    builder
        .name(format!("{}#{}", author.name, author.discriminator))
        .icon_url(match author.avatar_url() {
            Some(x) => x.to_owned(),
            None => author.default_avatar_url(),
        })
}
