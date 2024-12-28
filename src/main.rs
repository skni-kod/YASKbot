mod commands;
mod database;
mod tasks;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use tasks::{OtherTask, SimpleTask, Task};
use tokio_cron_scheduler::JobScheduler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _ = database::establish_connection();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::TestCommand::execute_command()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(commands::Data {})
            })
        })
        .build();
    // Register jobs
    let sched = JobScheduler::new().await.unwrap();
    let tasks: Vec<Box<dyn Task>> = vec![Box::new(SimpleTask), Box::new(OtherTask)];
    for job in &tasks {
        sched.add(job.job()).await.unwrap();
    }
    sched.start().await.unwrap();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
