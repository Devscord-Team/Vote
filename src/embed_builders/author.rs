use serenity::{builder::CreateEmbedAuthor, model::user::User};

pub fn build(builder: &mut CreateEmbedAuthor, author: User) -> &mut CreateEmbedAuthor {
    builder
        .name(format!("{}#{}", author.name, author.discriminator))
        .icon_url(match author.avatar_url() {
            Some(x) => x.to_owned(),
            None => author.default_avatar_url(),
        })
}
