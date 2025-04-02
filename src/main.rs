//Import anything we need from folders and files using 'mod'.
mod commands;
mod events;
mod util;
mod handlers;

use std::{fs, path::Path};

use git2::Repository;
use handlers::event_handler::event_handler;
use ::serenity::all::GatewayIntents;
use shuttle_runtime::SecretStore;
use commands::{fun::{say::say_command, uc::uc_command}, info::{info::info_command, links::links_command, ping::ping_command, serverinfo::serverinfo_command}, moderation::{blacklist::blacklist_command, ready_channel::ready_channel_command}};
use poise::serenity_prelude as serenity;
use util::remove_unused_commands;

pub struct Data {} // User data, which is stored and accessible in all command invocations.
type Error = Box<dyn std::error::Error + Send + Sync>; //Main type used for error handling of commands.
type Context<'a> = poise::Context<'a, Data, Error>; //The context of the data being used.
//The main shuttle runtime function, allows it to use content from Secrets.toml and deploy using shuttle.

fn get_commit() {
    let repo_url = "https://github.com/kagaries/Marle-Rust.git";
    let repo_path = "/tmp/git2-rs";
    let path = Path::new(repo_path);
    if path.exists() {
        // Remove the existing directory to ensure it's empty
        if let Err(e) = fs::remove_dir_all(path) {
            panic!("Failed to clear directory: {}", e);
        }
    }
    let repo = match Repository::clone(repo_url, repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };
    let head = match repo.head() {
        Ok(head) => head,
        Err(e) => panic!("Failed to get head: {}", e),
    };
    let oid = head.target().unwrap();
    let commit = match repo.find_commit(oid) {
        Ok(commit) => commit,
        Err(e) => panic!("Failed to find commit: {}", e),
    };
    let commit_hash = commit.summary().unwrap();
    std::env::set_var("LAST_COMMIT", commit_hash );
}


#[shuttle_runtime::main]
async fn serenity( #[shuttle_runtime::Secrets] secrets: SecretStore, ) -> shuttle_serenity::ShuttleSerenity {
    get_commit();
    std::env::set_var("RUST_BACKTRACE", "1");
    //Configure the client with your Discord bot token in the environment.
    let token: String = secrets.get("DISCORD_TOKEN").unwrap();
    
    //The gateway intents we want and/or will use for events and commands.
    //non_privileged gets all intents not considered privileged by discord.
    let intents: GatewayIntents = GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    //Sets the "DB_LINK" field for the process environment.
    std::env::set_var("DB_LINK", secrets.get("DB_LINK").unwrap());

    //A vector array of command functions to be put into the framework.
    let cmds: Vec<poise::Command<Data, Box<dyn std::error::Error + Send + Sync>>> = vec![
        serverinfo_command(),
        links_command(),
        uc_command(),
        ping_command(),
        blacklist_command(),
        info_command(),
        say_command(),
        ready_channel_command()
    ];

    //The poise framework to load the commands and event handler for use with the bot.
    let framework: poise::Framework<Data, Box<dyn std::error::Error + Send + Sync>> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("m&".into()),
                ..Default::default()
            },
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
                remove_unused_commands(ctx, framework).await?;
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