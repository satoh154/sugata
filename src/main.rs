use std::{collections::HashMap};
use tokio::sync::Mutex;
use poise::serenity_prelude as serenity;

mod messenger;
mod session;
mod commands;
use commands::*;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    params_holder: Mutex<Vec<HashMap<String, HashMap<String, usize>>>>,
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD TOKEN");
    let framework = poise::Framework::build()
        .options(poise::FrameworkOptions {
            commands: vec![cm(), skill(), new(), insan(), dice(), sdice(), register()],
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework|{
            Box::pin(async move { 
                Ok(Data {
                    params_holder: Mutex::new(Vec::new())
                }) 
            })
        });

    framework.run().await.unwrap();
}
