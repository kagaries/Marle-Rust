use std::collections::HashSet;
use poise::serenity_prelude::Context;
use serenity::all::{CacheHttp, Command};

pub async fn remove_unused_commands(ctx: &Context) -> serenity::Result<()> {
    //Grabs the list of registered global commands that exist on the bot.
    let existing_commands: Vec<Command> = Command::get_global_commands(ctx.http()).await?;

    //Convert current bot commands into a set of their names
    let active_command_names: HashSet<_> = vec![
        "age",
        "serverinfo",
        "links",
        "uc",
        "ping_command",
        "timeout",
        "kick",
        "ban"
    ]
    .into_iter()
    .collect();

    //Iterates the existing commands list and checks if the command is not present in the active command names.
    //If the command is not present, then we delete that command.
    //Note: This is a very very crude way of doing it, please try to find a different way.
    for command in existing_commands {
        if !active_command_names.contains(command.name.as_str()) {
            Command::delete_global_command(ctx.http(), command.id).await?;
        }
    }
    Ok(())
}
