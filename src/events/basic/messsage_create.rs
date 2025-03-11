use std::env;

use poise::serenity_prelude as serenity;
use async_postgres::connect;
use ::serenity::all::CacheHttp;
use tokio::spawn;

//The event called from the event_handler.
//Events can have different variables, and will most likely never match each other.
pub async fn handle_message_event(_ctx: &serenity::Context, message : &serenity::Message) {

    if message.content.starts_with("!blacklist ") {
        if message.author.id.to_string() == "536076925706305536" {
            let db_url = match env::var("DB_LINK") {
                Ok(url) => url,
                Err(_) => return,
            };
        
            let (client, conn) = match connect(db_url.parse().expect("Invalid DB URL")).await {
                Ok((client, conn)) => (client, conn),
                Err(_) => return,
            };

            spawn(conn);

            let arg: Vec<&str> = message.content.split_whitespace().collect();

            if let Some(_arg) = arg.get(1) {
                if arg.get(1).unwrap() == &"add" {
                    if message.guild(&_ctx.cache).is_none() {
                        return
                    } else {
                        let _ = client.execute("INSERT INTO blacklist (id) VALUES ($1)", &[&arg.get(2).unwrap().to_string()]).await;

                        let _ = message.reply(_ctx.http(), "Done").await;
                    }
                }

                if arg.get(1).unwrap() == &"remove" {
                    if message.guild(&_ctx.cache).is_none() {
                        return
                    } else {
                        let _ = client.execute("DELETE FROM blacklist WHERE id = $1", &[&arg.get(2).unwrap().to_string()]).await;

                        let _ = message.reply(_ctx.http(), "Done").await;
                    }
                }
            }
        } else {
            return
        }
    }

    //println!("Logged in as {}", data_about_bot.user.name);

    //The channel id of where we want to send our message for this ready event.
    //This should be changed to a channel id it has access to, or be changed to not send a message.

    //let channel_id: ChannelId = ChannelId::new(898665447476588625);

    //Attempts to send the message "Bot is online!" to the target channel id.
    //On error it prints "Error sending message: " along with the reason for the error.

    /*
    if let Err(why) = channel_id.say(ctx.http(), "Bot is online!").await {
        println!("Error sending message: {:?}", why);
    }
    */
}