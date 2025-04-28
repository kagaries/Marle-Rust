use crate::{Error, Context};

#[poise::command(prefix_command, rename = "test")]
pub async fn test_command(ctx: Context<'_>, #[rest] message: String) -> Result<(), Error> {
    ctx.say(format!("You said: {}", message)).await?;
    Ok(())
}
