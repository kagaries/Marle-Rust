use std::env;

use async_postgres::connect;
use chrono::Local;
use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateEmbedFooter};
use tokio::spawn;
use crate::{Error as OtherError, Context};

#[poise::command(slash_command, rename = "uc", subcommands("execute", "create", "remove", "get"))]
pub async fn uc_command(
    _ctx: Context<'_>
) -> Result<(), OtherError> {
    Ok(())
}

#[poise::command(slash_command, description_localized("en-US", "Executes a user command"))]
pub async fn execute(
    ctx: Context<'_>,
    command: String
) -> Result<(), OtherError> { 
    let (client, conn) = connect(env::var("DB_LINK").unwrap().parse()?).await?;

    spawn(conn);
    
    let rows = client.query("SELECT * FROM commands WHERE name = $1", &[&command]).await?;

    if let Some(row) = rows.get(0) {
        let to_send: String = row.get(2);
        let uses: i32 = row.get(5);

        let new_uses: i32 = uses + 1;

        client.execute("UPDATE commands SET uses = $1 WHERE name = $2", &[&&new_uses, &command]).await?;

        ctx.say(&to_send).await?;
    } else {
        ctx.say(format!("Unable to find command: {}", command)).await?;
    }


    Ok(())
}
#[poise::command(slash_command, description_localized("en-US", "Creates a new user command"))]
pub async fn create(
    ctx: Context<'_>,
    #[min_length = 1] #[max_length = 50] name: String,
    #[min_length = 1] #[max_length = 1000] sends: String,
    #[min_length = 1] #[max_length = 250] description: String
) -> Result<(), OtherError> { 
    let (client, conn) = connect(env::var("DB_LINK").unwrap().parse()?).await?;

    spawn(conn);

    let rows = client.query("SELECT * FROM commands WHERE name = $1", &[&name]).await?;

    let total_num_of_commands: i64 = client.query_one("SELECT COUNT(*) AS exact_count FROM commands WHERE author = $1", &[&ctx.author().id.get().to_string()]).await?.get(0);

    if total_num_of_commands >= 100 {
        ctx.say("You've created too many commands!").await?;
        return Ok(());
    }

    if let Some(_row) = rows.get(0) {
        ctx.say("Command already exists!").await?;
    } else {
        let now = Local::now();
        let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();

        client.execute("INSERT INTO commands (name, author, toSend, description, created, uses) VALUES ($1, $2, $3, $4, $5, 0)", &[&name, &ctx.author().id.get().to_string(), &sends, &description, &formatted]).await?;

        ctx.say(format!("Added Command: {}", name)).await?;
    }

    Ok(())
}
#[poise::command(slash_command, description_localized("en-US", "Removes a user command you've created"))]
pub async fn remove(
    ctx: Context<'_>, 
    command: String
) -> Result<(), OtherError> {

    let (client, conn) = connect(env::var("DB_LINK").unwrap().parse()?).await?;

    spawn(conn);
    
    let rows = client.query("SELECT * FROM commands WHERE name = $1", &[&command]).await?;

    if let Some(row) = rows.get(0) {
        let name: String = row.get(0);
        let author: String = row.get(1);

        if ctx.author().id.get().to_string().contains(&author) {
            client.execute("DELETE FROM commands WHERE name = $1", &[&command]).await?;

            ctx.say(format!("Deleted: {}", name)).await?;
        } else {
            ctx.say("You do not own this command.").await?;
        }
    } else {
        ctx.say(format!("Unable to find command: {}", command)).await?;
    }

    Ok(()) 
}
#[poise::command(slash_command, description_localized("en-US", "Grabs info about a user command"))]
pub async fn get(
    ctx: Context<'_>, 
    command: String
) -> Result<(), OtherError> {

    let (client, conn) = connect(env::var("DB_LINK").unwrap().parse()?).await?;

    spawn(conn);
    
    let rows = client.query("SELECT * FROM commands WHERE name = $1", &[&command]).await?;

    if let Some(row) = rows.get(0) {
        let name: String = row.get(0);
        let author: String = row.get(1);
        let to_send: String = row.get(2);
        let description: String = row.get(3);
        let created: String = row.get(4);
        let uses: i32 = row.get(5);

        let embed = CreateEmbed::new().title(name).description(description)
        .field("Sends:", to_send, true)
        .field("Author:", format!("<@{}>", author), true)
        .footer(CreateEmbedFooter::new(format!("Created: {} || Uses: {}", created, uses.to_string())));

        ctx.send(CreateReply::default().embed(embed)).await?;
    } else {
        ctx.say(format!("Unable to find command: {}", command)).await?;
    }

    Ok(()) 
}

