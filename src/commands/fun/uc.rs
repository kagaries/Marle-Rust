
use async_postgres::connect;
use chrono::{Local, NaiveDate};
use poise::CreateReply;
use serenity::{all::{CreateEmbed, CreateEmbedFooter}, FutureExt};
use tokio::{spawn, task};
use crate::{Error as OtherError, Context};
use dotenv_codegen::dotenv;

#[poise::command(slash_command, rename = "uc", subcommands("execute", "create", "remove", "get"))]
pub async fn uc_command(
    ctx: Context<'_>
) -> Result<(), OtherError> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn execute(
    ctx: Context<'_>,
    command: String
) -> Result<(), OtherError> { 
    let (client, conn) = connect(dotenv!("DB_LINK").parse()?).await?;

    spawn(conn);
    
    let rows = client.query("SELECT * FROM commands WHERE name = $1", &[&command]).await?;

    if let Some(row) = rows.get(0) {
        let to_send: String = row.get(2);
        let uses: i32 = row.get(5);

        let new_uses: i32 = uses + 1;

        client.execute("UPDATE commands SET uses = $1 WHERE name = $2", &[&&new_uses, &command]).await?;

        ctx.say(&to_send).await?;
    } else {
        println!("No results found for command: {}", command);
    }


    Ok(())
}
#[poise::command(slash_command)]
pub async fn create(
    ctx: Context<'_>,
    name: String,
    sends: String,
    description: String
) -> Result<(), OtherError> { 
    let (client, conn) = connect(dotenv!("DB_LINK").parse()?).await?;

    spawn(conn);

    let rows = client.query("SELECT * FROM commands WHERE name = $1", &[&name]).await?;

    if let Some(row) = rows.get(0) {
        println!("Command exists!");
    } else {
        let now = Local::now();
        let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();

        client.execute("INSERT INTO commands (name, author, toSend, description, created, uses) VALUES ($1, $2, $3, $4, $5, 0)", &[&name, &ctx.author().id.get().to_string(), &sends, &description, &formatted]).await?;

        ctx.say(format!("Added Command: {}", name)).await?;
    }

    Ok(())
}
#[poise::command(slash_command)]
pub async fn remove(
    ctx: Context<'_>, 
    command: String
) -> Result<(), OtherError> {

    let (client, conn) = connect(dotenv!("DB_LINK").parse()?).await?;

    spawn(conn);
    
    let rows = client.query("SELECT * FROM commands WHERE name = $1", &[&command]).await?;

    if let Some(row) = rows.get(0) {
        let name: String = row.get(0);
        let author: String = row.get(1);

        if (ctx.author().id.get().to_string().contains(&author)) {
            client.execute("DELETE FROM commands WHERE name = $1", &[&command]).await?;

            ctx.say(format!("Deleted: {}", name)).await?;
        } else {
            ctx.say("You do not own this command.").await?;
        }
    } else {
        println!("No results found for command: {}", command);
    }

    Ok(()) 
}
#[poise::command(slash_command)]
pub async fn get(
    ctx: Context<'_>, 
    command: String
) -> Result<(), OtherError> {

    let (client, conn) = connect(dotenv!("DB_LINK").parse()?).await?;

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
        println!("No results found for command: {}", command);
    }

    Ok(()) 
}

