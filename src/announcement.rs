use poise::{serenity_prelude as serenity};
use serenity::builder::CreateEmbed;
use crate::{Context, Error};

/// Send an announcement embed
#[poise::command(slash_command)]
pub async fn announce(
    ctx: Context<'_>,
    #[description = "Embed title"] title: String,
    #[description = "Embed description"] description: String,
) -> Result<(), Error> {
    let author = ctx.author();
    let author_id = ctx.author().id;
    let owner_id: u64 = std::env::var("owner_id")
        .expect("Missing owner_id")
        .parse()
        .expect("Invalid owner_id format");

    if author_id != owner_id {
        ctx.say("❌ You are not allowed to use this command.").await?;
        return Ok(());
    }

    let ac_id: u64 = std::env::var("announcement_channel_id")
        .expect("Missing announcement_channel_id")
        .parse()
        .expect("Invalid announcement_channel_id format");

    let embed = CreateEmbed::default()
        .title(title)
        .description(description)
        .author(
            serenity::CreateEmbedAuthor::new(&author.name)
            .icon_url(author.avatar_url().unwrap_or_else(|| author.default_avatar_url())))
        .color(0x62418d);

    let message = serenity::CreateMessage::new().embed(embed);

    let announcement_channel_id: serenity::ChannelId = serenity::ChannelId::new(ac_id);

    if let Err(err) = announcement_channel_id.send_message(&ctx.serenity_context().http, message).await {
        ctx.say(format!("❌ Failed to send message: {err}")).await?;
    } else {
        ctx.say("✅ Announcement sent.").await?;
    }

    Ok(())
}

