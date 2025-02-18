use std::time::Instant;

use poise::CreateReply;
use crate::{Error, Context};

//The slash command setup is made up of the #[poise::command(slash_command)] along with its variables,
//and the variables within the function.
//A command will always have a Context variable, with the context variable being the one you created in your crate (main.rs), it can have more arguments which will be the options for the slash commands.
//The 'Result<(), Error> {' should always have the Error as the one you created in the crate (main.rs).
#[poise::command(slash_command, description_localized("en-US", "Pong!"))]
pub async fn ping_command(
    ctx: Context<'_>
) -> Result<(), Error> {
    //Grabs the instant of the time when this function is called.
    let start_time: Instant = Instant::now();
    
    //Sends a message and is set as a variable to be edited later.
    let sent_msg: poise::ReplyHandle<'_> = ctx.say("Measuring ping...").await?;

    //Gets the elapsed instant time as milliseconds (u128).
    let latency: u128 = start_time.elapsed().as_millis();

    //Edits the message with a format!() to show the ms it took for the message to complete.
    sent_msg.edit(ctx, CreateReply::default().content(format!("Pong! ms: {}", latency))).await?;
    
    Ok(())
}