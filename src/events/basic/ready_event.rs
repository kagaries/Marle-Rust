use poise::serenity_prelude as serenity;
use ::serenity::all::{CacheHttp, ChannelId};

//The event called from the event_handler.
//Events can have different variables, and will most likely never match each other.
pub async fn handle_ready_event(ctx: &serenity::Context, data_about_bot : &serenity::Ready) {

    println!("Logged in as {}", data_about_bot.user.name);

    //The channel id of where we want to send our message for this ready event.
    //This should be changed to a channel id it has access to, or be changed to not send a message.

    let channel_id: ChannelId = ChannelId::new(898665447476588625);

    //Attempts to send the message "Bot is online!" to the target channel id.
    //On error it prints "Error sending message: " along with the reason for the error.

    if let Err(why) = channel_id.say(ctx.http(), "Marle is online!").await {
        println!("Error sending message: {:?}", why);
    }
}