use poise::serenity_prelude as serenity;
use crate::{events, Data, Error};
use events::basic::ready_event::handle_ready_event;

//Event handlers must be set up with these variables, the bot will error if it doesn't meet this.
pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    //Match an event with any listed below.
    //Any events not listed like the ready event will default to the _ doing nothing.
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            handle_ready_event(ctx, data_about_bot).await;
        }
        _ => {}
    }
    Ok(())
}