use std::borrow::Cow;
use image::{Rgba, RgbaImage};
use image::codecs::gif;
use image::ImageFormat::Gif;
use imageproc::drawing;
use imageproc::rect::Rect;
use ::gif::{Encoder};
use ::gif::Frame;

extern crate base64;

use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::{InteractionResponseType};
use serenity::model::channel::AttachmentType::{Bytes, File, Image};
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction};
use serenity::prelude::*;

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context) -> serenity::Result<()> {

    let mut encoder = Encoder::new(Vec::new(), 100, 100, &[]).unwrap();

    for i in 0..10 {
        let mut image = RgbaImage::new(100, 100);
        drawing::draw_filled_circle_mut(&mut image, (50, 50), 50, Rgba([i * 25, 0, 0, 255]));

        let mut frame = Frame::default();
        frame.width = 100;
        frame.height = 100;
        frame.buffer = Cow::Owned(image.into_raw());
        frame.delay = 100;
        encoder.write_frame(&frame).unwrap();
    }

    //TODO: Verify that the file is an image by saving it somewhere and checking the file type
    //TODO: Fix the file so he can be seen on discord

    // Send the image as a response
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.add_file(Bytes{
                    data: Cow::from(encoder.get_mut().to_vec()),
                    filename: "test.gif".to_string()
                }))
        })
        .await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("test").description("A command to test the bot")
}