use std::env;

use async_postgres::connect;
use poise::CreateReply;
use tokio::spawn;
use crate::{Error as OtherError, Context};

#[poise::command(slash_command, description_localized("en-US", "Set if the ready message should be sent"), rename = "ready_channel", required_permissions = "MANAGE_GUILD")]
pub async fn ready_channel_command(
    ctx: Context<'_>,
    remove: Option<bool>
) -> Result<(), OtherError> {
    let (client, conn) = connect(env::var("DB_LINK").unwrap().parse()?).await?;

    spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    let guild_id = ctx.guild().unwrap().id.get().to_string();
    let channel_id = ctx.channel_id().get().to_string();

    let table = client
        .query_one(
            "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'public' AND table_name = 'guilds')",
            &[],
        )
        .await?;

    let table_exists: bool = table.get(0);

    if !table_exists {
        client.execute(
            "CREATE TABLE guilds (
                id TEXT PRIMARY KEY,
                ready_channel TEXT
            )",
            &[],
        )
        .await?;
    }

    let guild = client.query("SELECT * FROM guilds WHERE id = $1", &[&guild_id]).await?;

    if guild.is_empty() {
        if remove.unwrap_or(false) == true {
            ctx.send(CreateReply::default().content("No channel has been set.")).await?;
            return Ok(())
        } else {
            client.execute(
                "INSERT INTO guilds (id, ready_channel) VALUES ($1, $2)",
                &[&guild_id, &channel_id],
            )
            .await?;
            ctx.send(CreateReply::default().content(format!("Set ready channel to <#{}>.", channel_id))).await?;
        }
        
    } else {
        if remove.unwrap_or(false) == true {
            client.execute(
                "UPDATE guilds SET ready_channel = '0' WHERE id = $1",
                &[&guild_id],
            )
            .await?;
            ctx.send(CreateReply::default().content("Disabled ready message.")).await?;
        } else {
            client.execute(
                "UPDATE guilds SET ready_channel = $1 WHERE id = $2",
                &[&channel_id, &guild_id],
            )
            .await?;
            ctx.send(CreateReply::default().content(format!("Set ready channel to <#{}>.", channel_id))).await?;
        }
    }

    Ok(())
}