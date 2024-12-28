// Commands are handlers for user action from server. This may also include scheduling tasks
// Commands are stateless by themselves, but may cause individual changes to database
pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
use poise::serenity_prelude as serenity;

use crate::database;

pub struct TestCommand {}
impl TestCommand {
    /// Displays your or another user's account creation date
    #[poise::command(slash_command, rename = "age")]
    pub async fn execute_command(
        ctx: Context<'_>,
        #[description = "Selected user"] user: Option<serenity::User>,
    ) -> Result<(), Error> {
        database::commands::servers::try_register_server(ctx.guild_id().unwrap().get() as i64)?;
        let u = user.as_ref().unwrap_or_else(|| ctx.author());
        let response = format!("{}'s account was created at {}", u.name, u.created_at());
        ctx.say(response).await?;
        Ok(())
    }
}
