use serenity::model::prelude::Message;
use serenity::prelude::*;

use crate::{voting, Handler};

pub async fn handle(handler: &Handler, ctx: Context, new_message: Message) {
    if new_message.author.bot {
        return;
    }
    //TODO from config
    let create_vote_channel_id: u64 = 636638300731604996;
    if new_message.channel_id.as_u64() != &create_vote_channel_id {
        return;
    }
    voting::a_create::handle(handler, ctx, new_message).await;
}
