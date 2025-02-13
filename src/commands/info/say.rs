use poise::CreateReply;
use crate::{main, Data, Error, Context};

#[poise::command(slash_command, description_localized("en-US", "Sends a message with the bot"))]
pub async fn say_command(
    ctx: Context<'_>,
    #[description = "text"] string: String,
) -> Result<(), Error> {
    if let Some(channel) = ctx.guild_channel().await {
        channel.say(ctx.http(), &string).await?;
    }

    ctx.send(CreateReply::default().content("Complete").ephemeral(true)).await?;
    
    Ok(())
}