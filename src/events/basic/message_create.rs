use poise::serenity_prelude as serenity;
use regex::Regex;
use ::serenity::all::{CacheHttp, EditMessage};

//The event called from the event_handler.
//Events can have different variables, and will most likely never match each other.
pub async fn handle_message_event(_ctx: &serenity::Context, message : &serenity::Message) {

    if message.guild_id != None {
        if message.guild_id.unwrap().get() == 1353524385192869928 {
            let regex = Regex::new(r"(https?:\/\/(?:www\.)?(x\.com|twitter\.com|reddit\.com|instagram\.com|tiktok\.com)\/[^\s]+)").unwrap();
            let mut links: Vec<String> = Vec::new();
                
            for cap in regex.find_iter(&message.content) {
                let url = cap.as_str();
    
                let modified_url = url
                    .replace("https://x.com", "https://fxtwitter.com")
                    .replace("https://twitter.com", "https://fxtwitter.com")
                    .replace("https://www.reddit.com", "https://rxddit.com")
                    .replace("https://reddit.com", "https://rxddit.com")
                    .replace("https://www.instagram.com", "https://ddinstagram.com")
                    .replace("https://instagram.com", "https://ddinstagram.com")
                    .replace("https://www.tiktok.com", "https://tfxktok.com")
                    .replace("https://tiktok.com", "https://tfxktok.com");
    
                links.push(modified_url);
            }
    
            let message_id = message.id;
            let channel_id = message.channel_id;
    
            if !links.is_empty() {
                let _ = channel_id.edit_message(_ctx.http(), message_id, EditMessage::new().suppress_embeds(true)).await;
                let _ = message.reply(_ctx.http(), links.join("\n")).await;
            }
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