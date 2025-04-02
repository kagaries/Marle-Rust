use poise::CreateReply;
use serenity::all::CreateMessage;
use crate::{Error, Context};

#[poise::command(slash_command, rename = "say")]
pub async fn say_command(
    ctx: Context<'_>,
    message: String
) -> Result<(), Error> {
    if ctx.author().id.to_string() == "536076925706305536" {
        ctx.send(CreateReply::default().content("ok!").ephemeral(true)).await?.delete(ctx).await?;
        ctx.channel_id().send_message(ctx.http(), CreateMessage::default().content(message)).await?;
    } else {
        ctx.send(CreateReply::default().content("Missing Permission").ephemeral(true)).await?;
        return Ok(())
    }

    Ok(())
}