use serenity::all::User;

use crate::{Error, Context};

#[poise::command(slash_command, guild_only, required_permissions = "BAN_MEMBERS", rename = "ban")]
pub async fn ban_command(
    ctx: Context<'_>,
    user: User,
    message_delete_day_amount: Option<u8>,
    reason: Option<String>
) -> Result<(), Error> {
    let guild = ctx.serenity_context().http.get_guild(ctx.guild_id().unwrap()).await?;
    
    let user_member = guild.member(ctx, user.id).await.unwrap();
    let bot_member = guild.member(ctx, ctx.framework().bot_id).await.unwrap();
    let author_member = guild.member(ctx, ctx.author()).await.unwrap();

    let user_highest_role = user_member.roles
    .iter()
    .filter_map(|role| guild.roles.get(role))
    .max_by_key(|role| role.position);

    let author_highest_role = author_member.roles
    .iter()
    .filter_map(|role| guild.roles.get(role))
    .max_by_key(|role| role.position);

    let bot_highest_role = bot_member.roles
    .iter()
    .filter_map(|role| guild.roles.get(role))
    .max_by_key(|role| role.position);

    if !guild.user_permissions_in(&ctx.guild_channel().await.unwrap(), &guild.member(ctx, ctx.framework().bot_id).await.unwrap()).ban_members() {
        ctx.say("Bot missing permission: ``Ban Members``").await?;
        return Ok(());
    }

    if author_highest_role < user_highest_role {
        ctx.say("User has higher role then you.").await?;
        return Ok(());
    }

    if bot_highest_role < user_highest_role {
        ctx.say("User has higher role then bot!").await?;
        return Ok(());
    }

    if message_delete_day_amount > Some(7) {
        ctx.say("You can only delete messages 7 days before!").await?;
        return Ok(());
    }

    guild.member(ctx, user.id).await?
        .ban_with_reason(ctx, message_delete_day_amount.unwrap_or(0), &reason.unwrap_or("No reason provided.".to_string())).await?;

    Ok(())
}