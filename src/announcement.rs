use poise::serenity_prelude as serenity;
use serenity::{async_trait, model::channel::Message};



pub struct AnnouncementHandler;

#[async_trait]
impl serenity::EventHandler for AnnouncementHandler {
    async fn message(&self, ctx: serenity::Context, msg: Message) {
        let admin_id: u64 = std::env::var("owner_id")
            .expect("Missing owner_id")
            .parse()
            .expect("Invalid owner_id format");

        let announcement_channel_id: u64 = std::env::var("announcement_channel_id")
            .expect("Missing announcement_channel_id")
            .parse()
            .expect("Invalid announcement_channel_id format");

        // Only handle DMs from the admin
        if msg.guild_id.is_none() && msg.author.id.get() == admin_id {
            // Parse message content
            let title = msg
                .content
                .lines()
                .find(|line| line.starts_with("Title: "))
                .map(|line| line.trim_start_matches("Title: ").trim());
            
            let description = msg
                .content
                .lines()
                .find(|line| line.starts_with("Description: "))
                .map(|line| line.trim_start_matches("Description: ").trim());

            if let (Some(title), Some(description)) = (title, description) {
                let channel_id = serenity::model::id::ChannelId::new(announcement_channel_id);
                
                let send_result = channel_id
                    .send_message(&ctx.http, serenity::builder::CreateMessage::new()
                        .embed(serenity::builder::CreateEmbed::new()
                            .title(title)
                            .description(description)
                            .color(0x62418d)
                        )
                    )
                    .await;

                if let Err(e) = send_result {
                    eprintln!("Failed to send announcement: {:?}", e);
                }
            }
        }
    }
}