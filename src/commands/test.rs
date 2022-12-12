

use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::{InteractionResponseType};
//use serenity::model::channel::AttachmentType::Image;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction};
use serenity::prelude::*;

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context) -> serenity::Result<()> {
    // Create a gif


    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("tutu"))
        })
        .await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("test").description("A command to test the bot")
}