// check images against c2pa
// accepts: jpeg, png, webp

#[poise::command(slash_command)]
pub async fn credential_image<U: sync, E>(ctx: poise::Context<'_, U, E>) -> Result<(), E> {
    // TODO: implement c2pa image credential
}
