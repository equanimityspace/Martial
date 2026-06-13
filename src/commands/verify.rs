// gets media from user and verifys with c2pa

use crate::Context;
use crate::Error;
use c2pa::{Context as C2paContext, Reader};
use futures::future::join_all;
use std::io::Cursor;

#[poise::command(slash_command, help_text_fn = "help_verify")]
pub async fn verify(
    ctx: Context<'_>,
    // users can upload multiple attachments in one command, so prepare to loop through them
    #[description = "media file(s) to verify"] files: Vec<serenity::model::prelude::Attachment>,
) -> Result<(), Error> {
    ctx.defer().await?;

    // if user didn't upload anything, inform them that they should try it
    if files.is_empty() {
        ctx.reply("Please attach at least one file when running verify")
            .await?;
        return Ok(());
    }

    let processing_tasks = files.into_iter().map(|file| {
        async move {
            // c2pa requires attachment byte info
            let file_data = file.download().await?;
            let content_type = file.content_type.as_deref().unwrap_or("");
            let stream = Cursor::new(file_data);

            // create reader
            let c2pa_context =
                C2paContext::new().with_settings(include_str!("../../config.toml"))?;
            let reader = Reader::from_context(c2pa_context).with_stream(content_type, stream)?;

            let json_manifest = reader.json().to_string();

            ctx.say(format!("Processed {}...", file.filename)).await?;

            Ok::<(String, String), Error>((file.filename, json_manifest))
        }
    });
    let results = join_all(processing_tasks).await;

    for result in results {
        match result {
            Ok((filename, json_string)) => {
                println!("File {} manifest data: {}", filename, json_string);
                ctx.reply(json_string).await?;
            }
            Err(e) => {
                println!("error: {}", e);
                ctx.reply("error! oopsie").await?;
            }
        }
    }
    Ok(())
}

fn help_verify() -> String {
    String::from(
        "\
        Run /verify with at least one image/video/audio file attached",
    )
}
