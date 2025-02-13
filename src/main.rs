use std::env;

mod commands;

use commands::say::say_command;
use poise::{serenity_prelude as serenity, ChoiceParameter, CreateReply};
use ::serenity::all::{CreateEmbed, CreateEmbedFooter, PremiumTier};

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, description_localized("en-US", "Sends a message with the bot"))]
async fn say(
    ctx: Context<'_>,
    #[description = "text"] string: String,
) -> Result<(), Error> {
    ctx.guild_channel().await.unwrap().say(ctx.http(), &string).await?;

    ctx.send(CreateReply::default().content("Complete").ephemeral(true)).await?;
    
    Ok(())
}

#[poise::command(slash_command)]
async fn serverinfo(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let boost_level = match ctx.guild().as_ref().unwrap().premium_tier {
        PremiumTier::Tier0 => "Level 0 (No Boosts)",
        PremiumTier::Tier1 => "Level 1",
        PremiumTier::Tier2 => "Level 2",
        PremiumTier::Tier3 => "Level 3",
        _ => "Unknown boost level",
    };

    let m = CreateReply::default();
    let embed = CreateEmbed::new()
    .title(ctx.guild().as_ref().unwrap().name.clone())
    .description(ctx.guild().as_ref().unwrap().description.as_deref().filter(|desc| !desc.is_empty()).unwrap_or("No description set."))
    .field("Owner:", format!("<@{}>", ctx.guild().as_ref().unwrap().owner_id.get().to_string()), true)
    .field("Created:", ctx.guild().as_ref().unwrap().id.created_at().to_string(), true)
    .field("Vanity:", ctx.guild().as_ref().unwrap().vanity_url_code.clone().unwrap_or("No vanity".to_string()).to_string(), true)
    .field("Boost Tier:", boost_level, true)
    .field("Role Amount:", ctx.guild().as_ref().unwrap().roles.capacity().to_string(), true)
    .field("Member Count:", ctx.guild().as_ref().unwrap().member_count.to_string(), true)
    .footer(CreateEmbedFooter::new(format!("ID: {}", ctx.guild().as_ref().unwrap().id.get().to_string()))); 

    ctx.send(m.embed(embed)).await?;

    Ok(())
}

#[derive(poise::ChoiceParameter)]
enum links {
    BetterDiscovery,
    Deepboken,
    DID,
    DreamGame
}

#[poise::command(slash_command)]
async fn links(
    ctx: Context<'_>,
    #[description = "link to be gotten"] link: links,
) -> Result<(), Error> {
    let response = match link {
        links::BetterDiscovery => "https://www.roblox.com/games/15317947079/better-discovery",
        links::Deepboken => "https://tenor.com/view/the-owl-deepwoken-gif-4915453637006314785",
        links::DID => "# What are systems?
                    Systems are what the body is called when there are multiple people within it. When you speak to a system, **you may not always be talking to the person you know**.

                    You may notice personality shifts and memory gaps, **alters do not remember what other alters were doing**, they only have their own memories, although depending on the system they may share memories, it varies

                    # What are alters?
                    Alters form when a child around the ages 5-10 experiences trauma, during this age you are forming your many personalities, trauma halts this process and causes there to be multiple people rather than all the personalities merging, it should be noted they all have different names and behave differently

                    # common terms
                    there are many system terms, but fronting/cofronting and headspace are commonly used
                    > - fronting: when an alter takes control of the body (like the driver in a car)
                    > - co-fronting: when an alter is conscious but not in control of the body (like the passenger in the car)
                    > - headspace: a place in a systems mind where alters tend to be when they arent fronting

                    # What are roles?
                    Roles are specific things alters do in headspace or when fronting, i.e. littles, they’re pretty much just children despite the body’s age

                    if you wish to do your own research feel free to do so here https://did-research.org/

                    https://morethanone.info/",
        links::DreamGame => "https://www.roblox.com/games/5475056496/Dream-Game\nhttps://discord.gg/epicdepartment",
    };

    if link.name().to_string() == "DID" {
        ctx.send(CreateReply::default().content(response)).await?;
    } else {
        ctx.say(response).await?;
    };
    Ok(())
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    std::env::set_var("DISCORD_TOKEN", "MTE2MTQyMzg2MzQ4MjE1OTIzNA.G0Zxd0.FzvuVHTe5yUHya__kKu0URaXzAohQWEDnkJjzU");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), say(), serverinfo(), links(), say_command()],
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