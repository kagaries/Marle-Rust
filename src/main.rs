mod commands;

use shuttle_runtime::SecretStore;
use commands::{fun::uc::uc_command, info::{age::age_command, links::links_command, say::say_command, serverinfo::serverinfo_command}};
use poise::serenity_prelude as serenity;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn serenity( #[shuttle_runtime::Secrets] secrets: SecretStore,) -> shuttle_serenity::ShuttleSerenity {
    //Configure the client with your Discord bot token in the environment.
    let token = secrets.get("DISCORD_TOKEN").unwrap();
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age_command(), serverinfo_command(), links_command(), say_command(), uc_command()],
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
    let client = serenity::Client::builder(&token, intents)
        .framework(framework)
        .await
        .expect("err");
        

    Ok(client.into())
}