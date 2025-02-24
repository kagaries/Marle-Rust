use poise::serenity_prelude::Context;
use serenity::all::{CacheHttp, Command};

use crate::Data;

pub async fn remove_unused_commands(ctx: &Context, framework: &poise::Framework<Data, Box<dyn std::error::Error + Send + Sync>>) -> serenity::Result<()> {
    let existing_commands: Vec<Command> = Command::get_global_commands(ctx.http()).await?;
    let commands_list = &framework.options().commands;

    for command in &existing_commands {
        let exists = commands_list.iter().any(|cmd| cmd.name == command.name);
        if exists {
            println!("Command '{}' exists", command.name);
        } else {
            println!("Command '{}' does not exist", command.name);
            Command::delete_global_command(ctx.http(), command.id).await?;
        }
    }
    Ok(())
}
