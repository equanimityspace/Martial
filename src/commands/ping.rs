// check latency
use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    match ctx
        .framework()
        .shard_manager
        .runners
        .lock()
        .await
        .get(&ctx.serenity_context().shard_id)
    {
        Some(runner) => {
            ctx.reply(stringify!(
                runner.latency.unwrap_or(std::time::Duration::ZERO)
            ))
            .await?;
        }
        None => {
            println!("current shard is not in shard_manager.runners, this shouldn't happen",);
            ctx.reply(stringify!(std::time::Duration::ZERO)).await?;
        }
    }
    Ok(())
}
