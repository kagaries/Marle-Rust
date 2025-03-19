use poise::serenity_prelude as serenity;
use ::serenity::all::CacheHttp;
use url::Url;

fn is_valid_url(input: &str) -> Result<(), String> {
    match Url::parse(input) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Invalid URL: {}", e)),
    }
}

//The event called from the event_handler.
//Events can have different variables, and will most likely never match each other.
pub async fn handle_reaction_add(ctx: &serenity::Context, reaction : &serenity::Reaction) {
    if let Some(referenced_message) = reaction.message(ctx.http()).await.unwrap().referenced_message {
        if referenced_message.author == reaction.member.clone().unwrap().user {
            if reaction.message(ctx.http()).await.unwrap().author.bot {
                if is_valid_url(&reaction.message(ctx.http()).await.unwrap().content).is_ok() {
                    reaction.message(ctx.http()).await.unwrap().delete(ctx.http()).await.ok();
                } else {
                    println!("Not a valid URL!");
                }
            }
        }
    }
}