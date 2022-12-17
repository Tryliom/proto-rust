extern crate base64;

use std::borrow::Cow;
use std::io::{Write};
use image::{Rgba, RgbaImage};
use imageproc::drawing;
use imageproc::rect::Rect;

use ffmpeg_next::codec::encoder::video::Encoder as VideoEncoder;
use ffmpeg_next::codec::encoder::encoder::Encoder;
use ffmpeg_next::codec::encoder::video::Video;
use ffmpeg_next::codec::context::Context as CodecContext;
use ffmpeg_next::format::{self, Pixel};
use ffmpeg_next::util::frame::Video as VideoFrame;

use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::{InteractionResponseType};
use serenity::model::channel::AttachmentType::{Bytes};
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction};
use serenity::prelude::*;

pub async unsafe fn run(command: &ApplicationCommandInteraction, ctx: &Context) -> serenity::Result<()> {

    // Create a new gif encoder from ffmpeg
    let mut encoder = VideoEncoder(Video(Encoder(CodecContext::new())));
    encoder.set_height(100);
    encoder.set_width(100);
    encoder.set_format(Pixel::RGBA);
    encoder.set_time_base((1, 20));
    encoder.set_bit_rate(1000000);
    encoder.set_quality(100);

    for i in 1..50 {
        let mut image = RgbaImage::new(100, 100);
        drawing::draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(100, 100), Rgba([255, 255, 255, 255]));
        drawing::draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(i as u32, i as u32), Rgba([255, 0, 0, 255]));

        let mut frame = VideoFrame::new(Pixel::RGBA, 100, 100);


    }

    //TODO: Verify that the file is an image by saving it somewhere and checking the file type
    //TODO: Fix the file so he can be seen on discord



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