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
    #[description = "media file(s) to verify"] files: Vec<poise::serenity_prelude::Attachment>,
) -> Result<(), Error> {
    // if user didn't upload anything, inform them that they should try it
    if files.is_empty() {
        ctx.reply("Please attach at least one file when running verify")
            .await?;
        return Ok(());
    }

    ctx.defer().await?;

    let processing_tasks = files.into_iter().map(|file| {
        async move {
            println!("downloading {}", file.filename);
            // c2pa requires attachment byte info
            let file_data = file.download().await?;
            println!("finished downloading");
            let content_type = file.content_type.as_deref().unwrap_or("");
            let stream = Cursor::new(file_data);
            println!("mime: {}", content_type);

            // create reader
            let c2pa_context =
                C2paContext::new().with_settings(include_str!("../../config.toml"))?;
            let reader = Reader::from_context(c2pa_context)
                .with_stream(content_type, stream)
                .unwrap();

            let json_manifest = reader.json().to_string();

            Ok::<(String, String), Error>((file.filename, json_manifest))
        }
    });
    let results = join_all(processing_tasks).await;

    for result in results {
        match result {
            Ok((filename, json_string)) => {
                println!("File {} manifest data: {}", filename, json_string);
                ctx.say("success!").await?;
            }
            Err(e) => {
                println!("error: {}", e);
                ctx.say("error! oopsie").await?;
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
