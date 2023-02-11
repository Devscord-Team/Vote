use serenity::model::prelude::Reaction;
use serenity::prelude::*;

use crate::{voting, Handler};

pub async fn handle(handler: &Handler, ctx: Context, add_reaction: Reaction) {
    let member = add_reaction.member.clone();
    if member.unwrap().user.unwrap().bot {
        return;
    }
    let create_vote_channel_id: u64 = 636638300731604996;
    if add_reaction.channel_id.as_u64() == &create_vote_channel_id {
        voting::b_confirm::handle(handler, ctx, add_reaction).await;
        return;
    }
    let verify_vote_channel_id: u64 = 639030207902384138;
    if add_reaction.channel_id.as_u64() == &verify_vote_channel_id {
        let vote_id = voting::c_verify::handle(handler, &ctx, add_reaction.clone()).await;
        if let Some(x) = vote_id {
            let guild_id = add_reaction.guild_id.unwrap();
            voting::d_publish::handle(handler, ctx, guild_id, x).await;
        }
        return;
    }
}
