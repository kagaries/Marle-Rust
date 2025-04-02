use std::{env, u64};

use async_postgres::connect;
use poise::serenity_prelude as serenity;
use ::serenity::all::{ActivityData, CacheHttp, ChannelId};
use tokio::spawn;

//The event called from the event_handler.
//Events can have different variables, and will most likely never match each other.
pub async fn handle_ready_event(ctx: &serenity::Context, data_about_bot : &serenity::Ready) {
    //let commit = std::env::var("LAST_COMMIT").unwrap();

    let db_url = match env::var("DB_LINK") {
        Ok(url) => url,
        Err(_) => return,
    };

    let (client, conn) = match connect(db_url.parse().expect("Invalid DB URL")).await {
        Ok((client, conn)) => (client, conn),
        Err(_) => return,
    };

    spawn(conn);

    for ele in ctx.cache.guilds() {
        let guild = client.query("SELECT * FROM guilds WHERE id = $1", &[&ele.get().to_string()]).await.unwrap();

        if let Some(row) = guild.get(0) {
            let channel_id: String = row.get(1);
            if Some(channel_id.clone()).is_some() {
                let channel_num: u64 = channel_id.as_str().parse::<u64>().unwrap();

                if channel_num == 0 {
                    continue;
                }
                
                let actual_channel_id: ChannelId = ChannelId::new(channel_num);

                if let Err(why) = actual_channel_id.say(ctx.http(), format!("Marle is online! Last Commit: {}", std::env::var("LAST_COMMIT").unwrap())).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    println!("Logged in as {}", data_about_bot.user.name);

    ctx.set_presence(Some(ActivityData::custom("Hi")), serenity::OnlineStatus::Online);
}