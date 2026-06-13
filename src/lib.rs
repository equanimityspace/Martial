pub mod commands;
pub mod reader;

pub struct Data {}
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
