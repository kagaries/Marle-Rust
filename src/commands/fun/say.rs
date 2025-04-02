use serenity::all::CreateMessage;
use crate::{Error, Context};

#[poise::command(prefix_command, rename = "say")]
pub async fn say_command(
    ctx: Context<'_>,
    #[rest] message: String
) -> Result<(), Error> {
    if ctx.author().id.to_string() == "536076925706305536" {
        ctx.channel_id().send_message(ctx.http(), CreateMessage::default().content(message)).await?;
    } else {
        return Ok(())
    }

    Ok(())
}