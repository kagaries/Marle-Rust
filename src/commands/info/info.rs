use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateEmbedAuthor};
use crate::{Error, Context};

#[poise::command(slash_command, description_localized("en-US", "Bot info embed thing"), rename = "info")]
pub async fn info_command(
    ctx: Context<'_>
) -> Result<(), Error> {
    let embed = CreateEmbed::new().title("Marle").author(CreateEmbedAuthor::new("Kagaries (@kagaries)")).thumbnail(ctx.framework().bot_id.to_user(ctx.http()).await.unwrap().avatar_url().unwrap()).description("Just a simple bot made in rust.");

    ctx.send(CreateReply::default().embed(embed)).await?;
    
    Ok(())
}