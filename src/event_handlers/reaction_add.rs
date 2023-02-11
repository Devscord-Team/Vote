use rust_i18n::t;
use serenity::futures::StreamExt;
use serenity::model::prelude::{ChannelId, GuildChannel, GuildId, Reaction, ReactionType};
use serenity::model::user::User;
use serenity::prelude::*;

use crate::{embed_builders, Handler};

pub async fn handle(handler: &Handler, ctx: Context, add_reaction: Reaction) {
    if add_reaction.member.unwrap().user.unwrap().bot {
        println!("Reaction added by bot");
        return;
    }
    let create_vote_channel_id: u64 = 636638300731604996;
    if add_reaction.channel_id.as_u64() != &create_vote_channel_id {
        return;
    }
    println!("Reaction added by user");

    let message_id = i64::try_from(add_reaction.message_id.as_u64().to_owned()).unwrap();
    let guild_id = add_reaction.guild_id.unwrap();
    let server_id = i64::try_from(guild_id.as_u64().to_owned()).unwrap();
    let entry = sqlx::query!(
        "SELECT Id 
        FROM Votes 
        WHERE IsApprovedByAuthor=0 AND ApprovedByAdminId=0 AND MessageID=? AND ServerID=?",
        message_id,
        server_id
    )
    .fetch_optional(&handler.database)
    .await
    .unwrap();
    let channel_id = add_reaction.channel_id;
    if let Some(record) = entry {
        on_author_confirm(handler, ctx, record.Id, guild_id, server_id, channel_id).await;
    } else {
        channel_id
            .say(&ctx.http, t!("vote_asked.confirmed_error"))
            .await
            .unwrap();
    }
}

async fn on_author_confirm(
    handler: &Handler,
    ctx: Context,
    entry_id: i64,
    guild_id: GuildId,
    server_id: i64,
    channel_id: ChannelId,
) {
    //TODO refactor
    //TODO from config
    let verification_channel_id: u64 = 639030207902384138;

    let channels = guild_id.channels(&ctx.http).await.unwrap();

    let verification_channel = channels
        .iter()
        .find(|(channel_id, _)| channel_id.as_u64() == &verification_channel_id);

    if let Some((verification_channel_id, verification_channel)) = verification_channel {
        sqlx::query!(
            "UPDATE Votes 
            SET IsApprovedByAuthor=1 
            WHERE Id=? AND ServerID=?",
            entry_id,
            server_id
        )
        .execute(&handler.database) // < Where the command will be executed
        .await
        .unwrap();

        channel_id
            .say(&ctx.http, t!("vote_asked.confirmed", locale = "pl"))
            .await
            .unwrap();

        send_to_verification(
            handler,
            ctx,
            entry_id,
            guild_id,
            server_id,
            verification_channel_id,
            verification_channel,
        )
        .await;
    } else {
        channel_id
            .say(&ctx.http, t!("vote_asked.confirmed_error", locale = "pl"))
            .await
            .unwrap();
    }
}

async fn send_to_verification(
    handler: &Handler,
    ctx: Context,
    entry_id: i64,
    guild_id: GuildId,
    server_id: i64,
    verification_channel_id: &ChannelId,
    verification_channel: &GuildChannel,
) {
    let entry = sqlx::query!(
        "SELECT Author, AuthorID, Content 
        FROM Votes
        WHERE Id=? AND ServerID=?",
        entry_id,
        server_id
    )
    .fetch_one(&handler.database)
    .await
    .unwrap();

    let author_id_u64 = u64::try_from(entry.AuthorID.to_owned()).unwrap();
    let user = get_user_by_id(&ctx, guild_id, author_id_u64).await.unwrap();

    let sent_message_id = verification_channel
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.author(|a| embed_builders::author::build(a, user))
                    .title(t!(
                        "vote_waiting_for_verify.title",
                        locale = "pl",
                        approve_emoji = "✅"
                    ))
                    .description(&entry.Content)
            })
        })
        .await
        .expect("Error sending message")
        .id;

    verification_channel_id
        .create_reaction(
            &ctx.http,
            sent_message_id,
            ReactionType::Unicode("✅".to_string()),
        )
        .await
        .expect("Error adding reaction to message");
}

async fn get_user_by_id(ctx: &Context, guild_id: GuildId, user_id: u64) -> Option<User> {
    let mut members = guild_id.members_iter(&ctx.http).boxed();
    while let Some(member) = members.next().await {
        let unwraped = member.unwrap();
        if unwraped.user.id.as_u64() == &user_id {
            return Some(unwraped.user.to_owned());
        }
    }
    return None;
}
