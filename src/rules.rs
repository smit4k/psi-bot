use poise::{serenity_prelude as serenity, CreateReply};
use serenity::builder::CreateEmbed;
use crate::{Context, Error};



#[poise::command(prefix_command)]
pub async fn rules(ctx: Context<'_>) -> Result<(), Error> {
    let welcome_embed = CreateEmbed::default()
        .title("Welcome to the Conversia Support server!")
        .description("We're glad to have you here! Please take a moment to read the rules below so we can all enjoy a helpful and respectful community.")
        .color(0x62418d);

    let rules_embed = CreateEmbed::default()
        .title("Rules")
        .description("Read these rules carefully, if you do not follow these rules, we have the right to take action.")
        .field("**1. Be Respectful**", "Be respectful of others and their time, be kind. Racism, homophobia, hate speech, discrimination, or any form of bullying is not tolerated. Follow Discord's [Terms of Service](https://discord.com/terms) and [Community Guidelines](https://discord.com/guidelines).", false)
        .field("**2. No NSFW content**", "Absolutely no NSFW content is allowed.", false)
        .field("**3. No advertising**", "Do not advertise, this includes sharing links to other servers.", false)
        .field("**4. No spamming**", "Repetitive messages, extremely long messages, and mass pings are not welcome.", false)
        .field("**5. No selfbots/userbots**", "We do not support them, and we do not want to see them.", false)
        .field("**6. No politics/religion**", "This is not a debate club, this is a support server, we don't want to see any politics/religion discussions.", false)
        .field("**7. English only**", "This is an english speaking server, if you speak a different language, you are expected to provide a translation.", false)
        .field("**8. Use text channels accordingly**", "Please try to stay on topic for the text channel that you are in.", false)
        .color(0x62418d);

    let contact_staff_embed = CreateEmbed::default()
        .title("Contacting Server Staff")
        .description("If you see a problem with a moderator, contact <@562359659391090689>.\n\nIf you think someone is breaking the rules, please mention <@&1384592486135234642>")
        .color(0x62418d);
        
    let reply = CreateReply::default()
        .embed(welcome_embed)
        .embed(rules_embed)
        .embed(contact_staff_embed);
    
    ctx.send(reply).await?;
    Ok(())

}