use std::borrow::Cow;
use image::{Frame, Rgb, RgbImage};
use image::codecs::gif;
use image::ImageFormat::Gif;
use imageproc::drawing;
use imageproc::rect::Rect;
use ::gif::{Encoder};

use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::{InteractionResponseType};
//use serenity::model::channel::AttachmentType::Image;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction};
use serenity::prelude::*;

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context) -> serenity::Result<()> {

    let mut buffer = Vec::new();
    let mut encoder = Encoder::new(&mut buffer, 100, 100, &[]).unwrap();

    for i in 0..10 {
        let mut frame = Frame::default();
        frame.width = 100;
        frame.height = 100;

        let mut image = RgbImage::new(100, 100);
        drawing::draw_filled_circle_mut(&mut image, (50, 50), 50, Rgb([i * 25, 0, 0]));

        frame.buffer = Cow::from(image.to_vec());
        frame.delay = 100;
        encoder.write_frame(&frame).unwrap();
    }

    //TODO: Convert Vec to Attachment

    // Send the image as a response
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("tutu") | message.add_file(&mut buffer))
        })
        .await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("test").description("A command to test the bot")
}