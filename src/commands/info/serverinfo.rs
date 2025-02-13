use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateEmbedFooter, PremiumTier};
use crate::{Error, Context};

#[poise::command(slash_command, rename = "serverinfo")]
pub async fn serverinfo_command(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let boost_level = match ctx.guild().as_ref().unwrap().premium_tier {
        PremiumTier::Tier0 => "Level 0 (No Boosts)",
        PremiumTier::Tier1 => "Level 1",
        PremiumTier::Tier2 => "Level 2",
        PremiumTier::Tier3 => "Level 3",
        _ => "Unknown boost level",
    };

    let m = CreateReply::default();
    let embed = CreateEmbed::new()
    .title(ctx.guild().as_ref().unwrap().name.clone())
    .description(ctx.guild().as_ref().unwrap().description.as_deref().filter(|desc| !desc.is_empty()).unwrap_or("No description set."))
    .field("Owner:", format!("<@{}>", ctx.guild().as_ref().unwrap().owner_id.get().to_string()), true)
    .field("Created:", ctx.guild().as_ref().unwrap().id.created_at().to_string(), true)
    .field("Vanity:", ctx.guild().as_ref().unwrap().vanity_url_code.clone().unwrap_or("No vanity".to_string()).to_string(), true)
    .field("Boost Tier:", boost_level, true)
    .field("Role Amount:", ctx.guild().as_ref().unwrap().roles.capacity().to_string(), true)
    .field("Member Count:", ctx.guild().as_ref().unwrap().member_count.to_string(), true)
    .footer(CreateEmbedFooter::new(format!("ID: {}", ctx.guild().as_ref().unwrap().id.get().to_string()))); 

    ctx.send(m.embed(embed)).await?;

    Ok(())
}