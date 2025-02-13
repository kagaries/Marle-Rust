use chrono::Local;
use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage};

#[poise::command(slash_command, rename = "confess")]
pub async fn confess_command(
    _ctx: crate::Context<'_>,
    string: String,
) -> Result<(), crate::Error> {
    let now = Local::now();
    let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let embed = CreateEmbed::new()
    .title("Confession")
    .description(format!("'{}'", string))
    .footer(CreateEmbedFooter::new(formatted));

    if let Some(channel) = _ctx.guild_channel().await {
        channel.send_message(_ctx.http(), CreateMessage::new().add_embed(embed)).await?;
    }

    _ctx.send(CreateReply::default().content("Complete").ephemeral(true)).await?;

    Ok(())
}