use rust_i18n::t;
use serenity::model::prelude::{ChannelId, GuildChannel, GuildId, Reaction};
use serenity::prelude::*;

use crate::Handler;

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
    let entry = sqlx::query!(
        "SELECT Id 
            FROM Votes 
            WHERE IsApprovedByAuthor=0 AND ApprovedByAdminId=0 AND MessageID=?",
        message_id
    )
    .fetch_optional(&handler.database)
    .await
    .unwrap();
    let channel_id = add_reaction.channel_id;
    if let Some(record) = entry {
        on_author_confirm(
            handler,
            ctx,
            record.Id,
            add_reaction.guild_id.unwrap(),
            channel_id,
        )
        .await;
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
    channel_id: ChannelId,
) {
    //TODO refactor
    let approve_channel_id: u64 = 639030207902384138;

    let channels = guild_id.channels(&ctx.http).await.unwrap();

    let approve_channel = channels
        .iter()
        .find(|(channel_id, _)| channel_id.as_u64() == &approve_channel_id);

    if let Some((_, channel_guild)) = approve_channel {
        sqlx::query!(
            "UPDATE Votes 
        SET IsApprovedByAuthor=1 
        WHERE Id=?",
            entry_id
        )
        .execute(&handler.database) // < Where the command will be executed
        .await
        .unwrap();

        channel_id
            .say(&ctx.http, t!("vote_asked.confirmed", locale = "pl"))
            .await
            .unwrap();

        send_to_verification(handler, ctx, channel_guild).await;
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
    verification_channel: &GuildChannel,
) {
}
