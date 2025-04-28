//Import anything we need from folders and files using 'mod' and 'use'.
mod commands;
mod events;
mod util;
mod handlers;

use std::{fs, path::Path};

use git2::Repository;

use handlers::event_handler::event_handler;

use ::serenity::all::GatewayIntents;
use shuttle_runtime::SecretStore;
use poise::serenity_prelude as serenity;

use commands::{fun::uc::uc_command, info::{info::info_command, latest_commit::latest_commit_command, links::links_command, ping::ping_command, serverinfo::serverinfo_command}, moderation::{blacklist::blacklist_command, test::test_command, ready_channel::ready_channel_command}};

use util::remove_unused_commands;

pub struct Data {} // User data, which is stored and accessible in all command invocations.
type Error = Box<dyn std::error::Error + Send + Sync>; //Main type used for error handling of commands.
type Context<'a> = poise::Context<'a, Data, Error>; //The context of the data being used.

//Sets the LAST_COMMIT var of the env as the latest commit summary for a git repo
fn get_commit() {
    //the repo url to get the latest commit of
    let repo_url = "https://github.com/kagaries/Marle-Rust.git";

    //the path we should store the repo in
    let repo_path = "/tmp/git2-rs";
    
    //the actually path it self
    let path = Path::new(repo_path);

    //check incase that path is already created, and remove it to allow it to be cloned into again
    if path.exists() {
        if let Err(e) = fs::remove_dir_all(path) {
            panic!("Failed to clear directory: {}", e);
        }
    }

    //Clone the repo from the url into the path
    let repo = match Repository::clone(repo_url, repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };

    //the head of the repo
    let head = match repo.head() {
        Ok(head) => head,
        Err(e) => panic!("Failed to get head: {}", e),
    };

    //Oid of the head target
    let oid = head.target().unwrap();

    //the commit of the oid
    let commit = match repo.find_commit(oid) {
        Ok(commit) => commit,
        Err(e) => panic!("Failed to find commit: {}", e),
    };

    //the summary of the commit as a &str
    let commit_summary = commit.summary().unwrap();
    
    //set this so i may be used elsewhere
    std::env::set_var("LAST_COMMIT", commit_summary );
}

//The main shuttle runtime function, allows it to use content from Secrets.toml and deploy using shuttle.
#[shuttle_runtime::main]
async fn serenity( #[shuttle_runtime::Secrets] secrets: SecretStore, ) -> shuttle_serenity::ShuttleSerenity {
    get_commit();

    //For help in fixing bugs
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
        ready_channel_command(),
        latest_commit_command(),
        test_command()
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