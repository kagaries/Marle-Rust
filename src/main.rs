//Import anything we need from folders and files using 'mod'.
mod commands;
mod events;
mod util;
mod handlers;

use handlers::event_handler::event_handler;
use ::serenity::all::GatewayIntents;
use shuttle_runtime::SecretStore;
use commands::{fun::{confess::confess_command, uc::uc_command}, info::{age::age_command, links::links_command, ping::ping_command, serverinfo::serverinfo_command}, moderation::{ban::ban_command, kick::kick_command, timeout::timeout_command}};
use poise::serenity_prelude as serenity;
use util::remove_unused_commands;

pub struct Data {} // User data, which is stored and accessible in all command invocations.
type Error = Box<dyn std::error::Error + Send + Sync>; //Main type used for error handling of commands.
type Context<'a> = poise::Context<'a, Data, Error>; //The context of the data being used.

//The main shuttle runtime function, allows it to use content from Secrets.toml and deploy using shuttle.
#[shuttle_runtime::main]
async fn serenity( #[shuttle_runtime::Secrets] secrets: SecretStore,) -> shuttle_serenity::ShuttleSerenity {
    //Configure the client with your Discord bot token in the environment.
    let token: String = secrets.get("DISCORD_TOKEN").unwrap();

    //The gateway intents we want and/or will use for events and commands.
    //non_privileged gets all intents not considered privileged by discord.
    let intents: GatewayIntents = serenity::GatewayIntents::non_privileged();

    //Sets the "DB_LINK" field for the process environment.
    std::env::set_var("DB_LINK", secrets.get("DB_LINK").unwrap());

    //A vector array of command functions to be put into the framework.
    let cmds: Vec<poise::Command<Data, Box<dyn std::error::Error + Send + Sync>>> = vec![
        age_command(),
        serverinfo_command(),
        links_command(),
        uc_command(),
        confess_command(),
        ping_command(),
        timeout_command(),
        kick_command(),
        ban_command()
    ];

    //The poise framework to load the commands and event handler for use with the bot.
    let framework: poise::Framework<Data, Box<dyn std::error::Error + Send + Sync>> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: cmds,
            event_handler: |
            ctx: &::serenity::prelude::Context,
            event: &serenity::FullEvent,
            framework: poise::FrameworkContext<'_,
            Data, Box<dyn std::error::Error + Send + Sync>>, data: &Data | {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup( |
            ctx: &::serenity::prelude::Context,
            _ready: &serenity::Ready, 
            framework: &poise::Framework<Data, Box<dyn std::error::Error + Send + Sync>> | {
            Box::pin(async move {
                remove_unused_commands(ctx).await?;
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // Build our client.
    let client: serenity::Client = serenity::Client::builder(&token, intents)
        .framework(framework)
        .await
        .expect("err");
        

    Ok(client.into())
}