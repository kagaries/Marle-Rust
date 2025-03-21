use poise::CreateReply;
use crate::{Error, Context};

#[derive(poise::ChoiceParameter)]
enum Links {
    BetterDiscovery,
    Deepboken,
    DID,
    DreamGame,
    CatGPT,
}

#[poise::command(slash_command, rename = "links", description_localized("en-US", "Find helpful links!"))]
pub async fn links_command(
    ctx: Context<'_>,
    #[description = "link to be gotten"] link: Links,
) -> Result<(), Error> {
    let response = match link {
        Links::CatGPT => "https://catgpt.wvd.io/",
        Links::BetterDiscovery => "https://www.roblox.com/games/15317947079/better-discovery",
        Links::Deepboken => "https://tenor.com/view/the-owl-deepwoken-gif-4915453637006314785",
        Links::DID => "# What are systems?
                    Systems are what the body is called when there are multiple people within it. When you speak to a system, **you may not always be talking to the person you know**.

                    You may notice personality shifts and memory gaps, **alters do not remember what other alters were doing**, they only have their own memories, although depending on the system they may share memories, it varies

                    # What are alters?
                    Alters form when a child around the ages 5-10 experiences trauma, during this age you are forming your many personalities, trauma halts this process and causes there to be multiple people rather than all the personalities merging, it should be noted they all have different names and behave differently

                    # common terms
                    there are many system terms, but fronting/cofronting and headspace are commonly used
                    > - fronting: when an alter takes control of the body (like the driver in a car)
                    > - co-fronting: when an alter is conscious but not in control of the body (like the passenger in the car)
                    > - headspace: a place in a systems mind where alters tend to be when they arent fronting

                    # What are roles?
                    Roles are specific things alters do in headspace or when fronting, i.e. littles, they’re pretty much just children despite the body’s age

                    if you wish to do your own research feel free to do so here https://did-research.org/

                    https://morethanone.info/",
        Links::DreamGame => "https://www.roblox.com/games/5475056496/Dream-Game\nhttps://discord.gg/epicdepartment",
    };

    ctx.send(CreateReply::default().content(response).ephemeral(true)).await?;

    Ok(())
}