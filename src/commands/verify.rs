// gets media from user and verifys with c2pa

use crate::modules::get_manifest::get_manifest;
use crate::modules::modal::build_verification_modal;
use crate::{Context, Error};

#[poise::command(slash_command, help_text_fn = "help_verify")]
pub async fn verify(
    ctx: Context<'_>,
    // users can upload multiple attachments in one command, so prepare to loop through them
    #[description = "media file(s) to verify"] attachments: Vec<
        poise::serenity_prelude::Attachment,
    >,
) -> Result<(), Error> {
    // if user didn't upload anything, inform them that they should try it
    if attachments.is_empty() {
        ctx.reply("Please attach at least one file when running verify")
            .await?;
        return Ok(());
    }

    ctx.defer().await?;

    // stupid hack to get the correct thumbnail in each embed
    //let summaries = get_manifest(attachments.clone()).await?;

    let mut count = 0;
    for attachment in attachments {
        let summary = get_manifest(&attachment).await?;

        let thumbnail = attachment;

        let modal = build_verification_modal(summary, thumbnail);
        // ctx.send accepts models but need to create reply first
        ctx.send(poise::CreateReply::default().embed(modal).reply(false))
            .await?;

        count = count + 1;
    }

    Ok(())
}

fn help_verify() -> String {
    String::from(
        "\
        Run /verify with at least one image/video/audio file attached",
    )
}
