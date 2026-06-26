// Displays general information on Content Credentials and c2pa

use crate::{Context, Error};
use chrono::Utc;
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    // Martial Description
    let embed_description = r#"Martial checks submitted media files for C2PA Content Credentials. When a file is signed with Content Credentials, its creation and editing history are preserved (including the use of any AI models from parterned corporations).

    If Martial does not find any Content Credentials, the use of an AI model to create/edit the media cannot be determined. If Martial does find data from Content Credentials, it will be able to determine whether or not the media was created or edited by AI.

    For more information, click the "Content Credentials" button."#;

    // create embed
    let mut info_embed = serenity::CreateEmbed::default();

    //info_embed = info_embed.thumbnail(include_str!("../resources/martial_bust.jpg"));
    info_embed = info_embed.title("About Martial");
    info_embed = info_embed.color(0x4C5C68);
    info_embed = info_embed.description("The Roman poet Martial was born in 40AD and is konwn for being among the first to speak out against plagairism. [Find out more](https://www.plagiarismtoday.com/2011/10/04/the-world%E2%80%99s-first-plagiarism-case/)");

    info_embed = info_embed.field("About", embed_description, false);

    // get timestamp
    let timestamp =
        serenity::Timestamp::parse(&Utc::now().to_rfc3339()).expect("Invalid Timestamp");

    info_embed = info_embed.timestamp(timestamp);

    let reply = {
        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new_link("https://contentauthenticity.org/")
                .label("Content Credentials")
                .style(poise::serenity_prelude::ButtonStyle::Success)
                .emoji('🔗'),
        ])];

        poise::CreateReply::default()
            .embed(info_embed)
            .components(components)
    };

    ctx.send(reply).await?;

    Ok(())
}
