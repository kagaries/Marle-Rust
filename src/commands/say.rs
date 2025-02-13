use poise::CreateReply;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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