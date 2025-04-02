use std::env;

use async_postgres::connect;
use chrono::Local;
use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateEmbedFooter, User};
use similar_string::find_best_similarity;
use tokio::spawn;
use crate::{Error as OtherError, Context};

#[poise::command(slash_command, rename = "uc", subcommands("execute", "create", "remove", "get", "wipe", "getall"))]
pub async fn uc_command(
    _ctx: Context<'_>
) -> Result<(), OtherError> {
    Ok(())
}

async fn execute_command_callback<'a>(
    _ctx: Context<'a>, 
    partial: &str
) -> Vec<String> {
    let db_url = match env::var("DB_LINK") {
        Ok(url) => url,
        Err(_) => return vec!["DB_LINK not set".to_string()],
    };

    let (client, conn) = match connect(db_url.parse().expect("Invalid DB URL")).await {
        Ok((client, conn)) => (client, conn),
        Err(_) => return vec!["Failed to connect to DB".to_string()],
    };

    spawn(conn);

    let rows = match client.query(
        "SELECT name FROM commands WHERE name ILIKE $1 LIMIT 25",
        &[&format!("%{}%", partial)],
    ).await {
        Ok(rows) => rows,
        Err(_) => return vec!["Query failed".to_string()],
    };
    
    rows.iter().map(|row| row.get::<_, String>(0)).collect()
}

async fn remove_command_callback<'a>(
    _ctx: Context<'a>, 
    partial: &str
) -> Vec<String> {
    let db_url = match env::var("DB_LINK") {
        Ok(url) => url,
        Err(_) => return vec!["DB_LINK not set".to_string()],
    };

    let (client, conn) = match connect(db_url.parse().expect("Invalid DB URL")).await {
        Ok((client, conn)) => (client, conn),
        Err(_) => return vec!["Failed to connect to DB".to_string()],
    };

    spawn(conn);

    let rows = match client.query(
        "SELECT name FROM commands WHERE name ILIKE $1 AND author = $2 LIMIT 25",
        &[&format!("%{}%", partial), &_ctx.author().id.to_string()],
    ).await {
        Ok(rows) => rows,
        Err(_) => return vec!["Query failed".to_string()],
    };
    
    rows.iter().map(|row| row.get::<_, String>(0)).collect()
}

#[poise::command(slash_command, description_localized("en-US", "Executes a user command"))]
pub async fn execute(
    ctx: Context<'_>,
    #[autocomplete = "execute_command_callback"]
    command: String,
) -> Result<(), OtherError> { 
    ctx.defer().await?;
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
        let options: Vec<String> = client.query("SELECT name FROM commands", &[])
        .await?
        .iter()
        .map(|row| {
            row.get::<_, String>(0).to_string()
        })
        .collect();

        let best_match = find_best_similarity(command, &options);
        let string_thing = best_match.unwrap().0;

        if !string_thing.is_empty() {
            ctx.say(format!("Unable to find command. Did you mean ``{}``?", string_thing)).await?;
        } else {
            ctx.say("Unable to find command.").await?;
        }
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

    let table2 = client.query_one(
        "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'public' AND table_name = 'commands')",
        &[]
    ).await?;

    let table_exists2: bool = table2.get(0);

    if table_exists2 != true {
        client.execute(
            "CREATE TABLE commands (
                name TEXT PRIMARY KEY,
                author TEXT,
                toSend TEXT,
                description TEXT,
                created TEXT,
                uses INTEGER
            )", &[]).await?;
    }

    let rows = client.query("SELECT * FROM commands WHERE name = $1", &[&name]).await?;

    let table = client.query_one(
        "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'public' AND table_name = 'blacklist')",
        &[]
    ).await?;

    let table_exists: bool = table.get(0);

    if table_exists == true {
        let blacklist = client.query("SELECT * FROM blacklist WHERE id = $1", &[&ctx.author().id.to_string()]).await?;

        if let Some(_blacklist) = blacklist.get(0) {
            ctx.send(CreateReply::default().content("You are blacklisted from creating more commands!").ephemeral(true)).await?;
            return Ok(());
        }
    } else {
        print!("done!");
        client.execute(
            "CREATE TABLE blacklist (
                id TEXT PRIMARY KEY
            )",
            &[],
        )
        .await?;
    }

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
    #[autocomplete = "remove_command_callback"]
    command: String
) -> Result<(), OtherError> {
    ctx.defer().await?;
    let (client, conn) = connect(env::var("DB_LINK").unwrap().parse()?).await?;

    spawn(conn);
    
    let rows = client.query("SELECT * FROM commands WHERE name = $1", &[&command]).await?;

    if let Some(row) = rows.get(0) {
        let name: String = row.get(0);
        let author: String = row.get(1);

        if ctx.author().id.get().to_string() == "536076925706305536" {
            client.execute("DELETE FROM commands WHERE name = $1", &[&command]).await?;

            ctx.say(format!("Admin Deleted: {}", name)).await?;
            
            return Ok(())
        }

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

#[poise::command(slash_command, description_localized("en-US", "Removes all user commands you've created"))]
pub async fn wipe(
    ctx: Context<'_>
) -> Result<(), OtherError> {
    ctx.defer().await?;
    let (client, conn) = connect(env::var("DB_LINK").unwrap().parse()?).await?;

    spawn(conn);
    
    let rows = client.query("SELECT * FROM commands WHERE author = $1", &[&ctx.author().id.to_string()]).await?;

    if !rows.is_empty() {
        client.execute("DELETE FROM commands WHERE author = $1", &[&ctx.author().id.to_string()]).await?;
    
        ctx.say("All commands for this author have been removed.").await?;
    } else {
        ctx.say("No commands found for this author.").await?;
    }

    Ok(()) 
}

#[poise::command(slash_command, description_localized("en-US", "Gets all the commands a user has created"))]
pub async fn getall(
    ctx: Context<'_>,
    user: User
) -> Result<(), OtherError> {
    ctx.defer().await?;
    let (client, conn) = connect(env::var("DB_LINK").unwrap().parse()?).await?;

    spawn(conn);
    
    let rows = client.query("SELECT * FROM commands WHERE author = $1", &[&user.id.to_string()]).await?;

    if rows.is_empty() {
        ctx.say("No commands found for this author.").await?;
        return Ok(());
    }

    let commands: Vec<String> = rows.iter()
        .map(|row| row.get::<_, String>(0))
        .collect();

    let command_list = commands.join("\n");

    if command_list.chars().count() > 4000 {
        ctx.send(CreateReply::default().content("Please wait for me to add support for high character count :3").ephemeral(true)).await?;
        return Ok(());
    }

    ctx.send(CreateReply::default().content(command_list).ephemeral(true)).await?;

    Ok(()) 
}

#[poise::command(slash_command, description_localized("en-US", "Grabs info about a user command"))]
pub async fn get(
    ctx: Context<'_>, 
    #[autocomplete = "execute_command_callback"]
    command: String
) -> Result<(), OtherError> {
    ctx.defer().await?;
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

