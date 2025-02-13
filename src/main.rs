mod commands;

use ::serenity::all::CacheHttp;
use shuttle_runtime::SecretStore;
use commands::{fun::uc::uc_command, info::{age::age_command, links::links_command, say::say_command, serverinfo::serverinfo_command}};
use poise::serenity_prelude as serenity;

use serenity::all::{Command, CommandId};
use std::collections::HashSet;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn remove_unused_commands(ctx: &serenity::Context) -> serenity::Result<()> {
    let existing_commands = Command::get_global_commands(ctx.http()).await?;

    // Convert current bot commands into a set of their names
    let active_command_names: HashSet<_> = vec![
        "age",
        "serverinfo",
        "links",
        "say_command",
        "uc"
    ]
    .into_iter()
    .collect();

    for command in existing_commands {
        if !active_command_names.contains(command.name.as_str()) {
            // Command exists in Discord but not in the bot, so delete it
            Command::delete_global_command(ctx.http(), command.id).await?;
            println!("Deleted old command: {}", command.name);
        }
    }
    Ok(())
}

#[shuttle_runtime::main]
async fn serenity( #[shuttle_runtime::Secrets] secrets: SecretStore,) -> shuttle_serenity::ShuttleSerenity {
    //Configure the client with your Discord bot token in the environment.
    let token = secrets.get("DISCORD_TOKEN").unwrap();
    let intents = serenity::GatewayIntents::non_privileged();

    std::env::set_var("DB_LINK", secrets.get("DB_LINK").unwrap());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age_command(), serverinfo_command(), links_command(), say_command(), uc_command()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                remove_unused_commands(ctx).await?;
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // Build our client.
    let client = serenity::Client::builder(&token, intents)
        .framework(framework)
        .await
        .expect("err");
        

    Ok(client.into())
}