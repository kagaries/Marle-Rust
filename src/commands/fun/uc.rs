use postgres::{Client, NoTls};
use serenity::FutureExt;
use tokio::task;
use crate::{Error, Context};
use dotenv_codegen::dotenv;

#[poise::command(slash_command, rename = "uc", subcommands("execute", "create", "remove", "get"))]
pub async fn uc_command(
    ctx: Context<'_>
) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn execute(
    ctx: Context<'_>,
    command: String
) -> Result<(), Error> { 
    let result = task::block_in_place(|| {
        let mut client = Client::connect(
            &dotenv!("DB_LINK"),
            NoTls
        )?;

        let rows = client.query(&format!("SELECT * FROM commands WHERE name = '{}'", command), &[])?;

        if !rows.is_empty() {
            // Return the data to send as a result
            let to_send: String = rows.get(0).map(|row| row.get::<_, String>(2)).unwrap_or_else(|| String::from("No response found."));
            Ok(to_send)
        } else {
            Err("Command not found.".into())
        }
    })?;

    // Now send the response asynchronously
    match result {
        Ok(to_send) => {
            // Use ctx.say to send the result back
            ctx.say(to_send).await?;
        }
        Err(e) => {
            // Handle the error by sending an appropriate message
            ctx.say(format!("Error: {}", e)).await?;
        }
    }

    Ok(())
}
#[poise::command(slash_command)]
pub async fn create(
    ctx: Context<'_>,
    arg: String
) -> Result<(), Error> { 
    task::block_in_place(|| {
        let mut client = Client::connect(
            &format!("host={} user={} password={}", dotenv!("DB_LINK"), dotenv!("DB_USER"), dotenv!("DB_PASSWORD")),
            NoTls
        )?;

        Ok::<(), Error>(())
    })?;
    Ok(())
}
#[poise::command(slash_command)]
pub async fn remove(
    ctx: Context<'_>, 
    arg: String
) -> Result<(), Error> {
    task::block_in_place(|| {
        let mut client = Client::connect(
            &format!("host={} user={} password={}", dotenv!("DB_LINK"), dotenv!("DB_USER"), dotenv!("DB_PASSWORD")),
            NoTls
        )?;

        Ok::<(), Error>(())
    })?;
    Ok(()) 
}
#[poise::command(slash_command)]
pub async fn get(
    ctx: Context<'_>, 
    arg: String
) -> Result<(), Error> {
    task::block_in_place(|| {
        let mut client = Client::connect(
            &format!("host={} user={} password={}", dotenv!("DB_LINK"), dotenv!("DB_USER"), dotenv!("DB_PASSWORD")),
            NoTls
        )?;

        Ok::<(), Error>(())
    })?;
    Ok(()) 
}

