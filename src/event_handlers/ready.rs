use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::Handler;

pub async fn handle(_: &Handler, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
}
