mod commands;

extern crate dotenv_codegen;

use dotenv_codegen::dotenv;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        Command::create_global_application_command(&ctx.http, |command| {
            commands::test::register(command)
        }).await.expect("Create command failed");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            //println!("Received command interaction: {:#?}", command);

            if command.data.name.as_str() == "test" {
                unsafe { commands::test::run(&command, &ctx).await.expect("Run command failed"); }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize ffmpeg
    ffmpeg_next::init().unwrap();
    // Configure the client with your Discord bot token in the environment.
    let token = dotenv!("DISCORD_TOKEN");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}