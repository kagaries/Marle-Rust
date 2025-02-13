use std::time::Instant;

use poise::CreateReply;
use crate::{Error, Context};

#[poise::command(slash_command, description_localized("en-US", "Pong!"))]
pub async fn ping_command(
    ctx: Context<'_>
) -> Result<(), Error> {

    let start_time = Instant::now();
    
    let sent_msg = ctx.say("Measuring ping...").await?;
    let latency = start_time.elapsed().as_millis();

    sent_msg.edit(ctx, CreateReply::default().content(format!("Pong! ms: {}", latency))).await?;
    
    Ok(())
}