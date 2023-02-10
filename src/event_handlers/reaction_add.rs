use serenity::model::prelude::Reaction;
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

    match entry {
        Some(x) => println!("Found message with ID {}", x.Id),
        None => println!("Not found"),
    }
}
