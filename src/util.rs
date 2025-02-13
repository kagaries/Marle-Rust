use std::collections::HashSet;

use poise::serenity_prelude::Context;
use serenity::all::{CacheHttp, Command};

pub async fn remove_unused_commands(ctx: &Context) -> serenity::Result<()> {
    let existing_commands = Command::get_global_commands(ctx.http()).await?;

    // Convert current bot commands into a set of their names
    let active_command_names: HashSet<_> = vec![
        "age",
        "serverinfo",
        "links",
        "say_command",
        "uc",
        "ping_command"
    ]
    .into_iter()
    .collect();

    for command in existing_commands {
        if !active_command_names.contains(command.name.as_str()) {
            // Command exists in Discord but not in the bot, so delete it
            Command::delete_global_command(ctx.http(), command.id).await?;
        }
    }
    Ok(())
}
