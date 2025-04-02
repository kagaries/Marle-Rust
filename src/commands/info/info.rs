use poise::CreateReply;
use serenity::all::{Color, CreateEmbed, CreateEmbedAuthor};
use crate::{Error, Context};

#[poise::command(slash_command, description_localized("en-US", "Bot info embed thing"), rename = "info")]
pub async fn info_command(
    ctx: Context<'_>
) -> Result<(), Error> {
    let embed = CreateEmbed::new()
    .title("Marle")
    .author(CreateEmbedAuthor::new("Kagaries (@kagaries)"))
    .thumbnail(ctx.framework().bot_id.to_user(ctx.http()).await.unwrap().avatar_url().unwrap())
    .color(Color::from_rgb(0, 230, 255))
    .description("Just a simple bot made in rust.\nhttps://github.com/kagaries/Marle-Rust\n\nReact to a formatted link message with '❌' if you want to delete it");

    ctx.send(CreateReply::default().embed(embed)).await?;
    
    Ok(())
}