use std::env;

use poise::CreateReply;

use crate::{Context, Error};

#[poise::command(slash_command, description_localized("en-US", "Bot info embed thing"), rename = "version")]
pub async fn latest_commit_command(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.send(CreateReply::default().content(format!("```\n{}\n```\nVersion: ``{}``", std::env::var("LAST_COMMIT").unwrap(), env!("CARGO_PKG_VERSION")))).await?;

    Ok(())
}