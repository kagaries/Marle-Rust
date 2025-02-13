use ::serenity::all::CacheHttp;
use serenity::all::ChannelId;
use poise::serenity_prelude as serenity;

pub async fn handle_ready_event(ctx: &serenity::Context, data_about_bot : &serenity::Ready) {
    let channel_id = ChannelId::new(898665447476588625);

     if let Err(why) = channel_id.say(ctx.http(), "Bot is online!").await {
        println!("Error sending message: {:?}", why);
    }

    println!("Logged in as {}", data_about_bot.user.name);
}