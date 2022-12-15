extern crate base64;

use std::borrow::Cow;
use std::io::{Write};
use image::{Rgba, RgbaImage};
use imageproc::drawing;
use imageproc::rect::Rect;
use ::gif::{Encoder};
use ::gif::Frame;

use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::{InteractionResponseType};
use serenity::model::channel::AttachmentType::{Bytes};
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction};
use serenity::prelude::*;

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context) -> serenity::Result<()> {

    let mut encoder = Encoder::new(Vec::new(), 100, 100, &[0xFF, 0x00]).unwrap();

    for i in 1..50 {
        let mut image = RgbaImage::new(100, 100);
        drawing::draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(100, 100), Rgba([255, 255, 255, 255]));
        drawing::draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(i as u32, i as u32), Rgba([255, 0, 0, 255]));

        let mut frame = Frame::default();
        frame.width = 100;
        frame.height = 100;
        frame.buffer = Cow::Owned(image.into_raw());
        frame.delay = 10;

        encoder.write_frame(&frame).unwrap();
    }

    //TODO: Verify that the file is an image by saving it somewhere and checking the file type
    //TODO: Fix the file so he can be seen on discord

    let data = Cow::from(encoder.get_mut().to_vec());

    // Save encoder data to file
    let mut file = std::fs::File::create("test.gif").unwrap();
    file.write_all(data.as_ref()).unwrap();

    // Send the image as a response
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.add_file(Bytes{
                    data,
                    filename: "test.gif".to_string()
                }))
        })
        .await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("test").description("A command to test the bot")
}