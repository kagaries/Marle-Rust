use poise::serenity_prelude as serenity;
use crate::{events, Data, Error};
use events::basic::ready_event::handle_ready_event;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            handle_ready_event(ctx, data_about_bot).await;
        }
        _ => {}
    }
    Ok(())
}