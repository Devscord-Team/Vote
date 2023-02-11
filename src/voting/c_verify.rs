use rust_i18n::t;
use serenity::{model::prelude::Reaction, prelude::Context};

use crate::Handler;

pub async fn handle(handler: &Handler, ctx: &Context, add_reaction: Reaction) -> Option<i64> {
    let message_id = i64::try_from(add_reaction.message_id.as_u64().to_owned()).unwrap();
    let guild_id = add_reaction.guild_id.unwrap();
    let server_id = i64::try_from(guild_id.as_u64().to_owned()).unwrap();
    let author_id = i64::try_from(
        add_reaction
            .member
            .unwrap()
            .user
            .unwrap()
            .id
            .as_u64()
            .to_owned(),
    )
    .unwrap();
    let entry = sqlx::query!(
        "SELECT Id 
        FROM Votes 
        WHERE IsApprovedByAuthor=1 AND ApprovedByAdminId=0 AND VerificationMessageID=? AND ServerID=?",
        message_id,
        server_id,
    )
    .fetch_optional(&handler.database)
    .await
    .unwrap();
    let channel_id = add_reaction.channel_id;

    if let Some(record) = entry {
        let entry_id = record.Id;
        sqlx::query!(
            "UPDATE Votes 
            SET ApprovedByAdminId=? 
            WHERE Id=? AND ServerID=?",
            author_id,
            entry_id,
            server_id
        )
        .execute(&handler.database) // < Where the command will be executed
        .await
        .unwrap();

        channel_id
            .say(
                &ctx.http,
                t!("vote_waiting_for_verify.approved", locale = "pl"),
            )
            .await
            .unwrap();
        return Some(entry_id);
    } else {
        channel_id
            .say(&ctx.http, t!("commons.survey_not_found", locale = "pl"))
            .await
            .unwrap();
    }

    return None;
}
