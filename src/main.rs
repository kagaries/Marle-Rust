use std::env;

mod commands;

use commands::info::{age::age_command, links::links_command, say::say_command, serverinfo::serverinfo_command};
use poise::{serenity_prelude as serenity, ChoiceParameter, CreateReply};

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    std::env::set_var("DISCORD_TOKEN", "MTE2MTQyMzg2MzQ4MjE1OTIzNA.G0Zxd0.FzvuVHTe5yUHya__kKu0URaXzAohQWEDnkJjzU");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age_command(), serverinfo_command(), links_command(), say_command()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // Build our client.
    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}