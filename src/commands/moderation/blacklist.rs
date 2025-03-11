use std::env;

use async_postgres::connect;
use serenity::all::User;
use tokio::spawn;
use crate::{Error, Context};

#[poise::command(prefix_command, rename = "blacklist")]
pub async fn blacklist_command(
    ctx: Context<'_>,
    action: String,
    user: User,
) -> Result<(), Error> {
    if ctx.author().id.to_string() == "536076925706305536" {
        let db_url = match env::var("DB_LINK") {
            Ok(url) => url,
            Err(_) => return Ok(()),
        };
    
        let (client, conn) = match connect(db_url.parse().expect("Invalid DB URL")).await {
            Ok((client, conn)) => (client, conn),
            Err(_) => return Ok(()),
        };

        spawn(conn);

            if action == "add" {
                client.execute("INSERT INTO blacklist (id) VALUES ($1)", &[&user.id.to_string()]).await?;

                ctx.reply("Done").await?;

            }

            if action == "remove" {
                client.execute("DELETE FROM blacklist WHERE id = $1", &[&user.id.to_string()]).await?;

                ctx.reply("Done").await?;
            }
    } else {
        return Ok(())
    }

    Ok(())
}