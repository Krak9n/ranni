use std::path::Path;
use std::fs;
use std::env;

extern crate ffmpeg_next as ffmpeg;
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use ffmpeg::format;

struct video {}

impl video {

  /* VIDEO TO ASCII */
  // ---------------
  pub fn video_to_ascii(path: &str) -> Result<(), E> {
     let args: Vec<String> = env::args()
      .collect();

    ffmpeg::init().unwrap();
    fs::create_dir("./frames-from-video");

    // ffmpeg command for extracting frames
    // ffmpeg -i myvideo.mp4 img%03d.jpg
    let mut video_path: &str = &args[1].clone();
    let mut input = format::input(&Path::new(video_path))
        .map_err(|e| anyhow::anyhow!("failed to open input file: {}", e))?;

    let stream = input
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or(anyhow::anyhow!("no video stream found"))?;
    
    let stream_index = stream.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(stream.parameters())
        .map_err(|e| anyhow::anyhow!("failed to create decoder context: {}", e))?;
    let mut decoder = context_decoder.decoder()
        .video()
        .map_err(|e| anyhow::anyhow!("failed to create video decoder: {}", e))?;

    let mut frame_index = 0;
    // Decode frames
    for (stream, packet) in input.packets() {
        if stream.index() == stream_index {
            decoder.send_packet(&packet)
                .map_err(|e| anyhow::anyhow!("failed to send packet to decoder: {}", e))?;
            
            println!("extracting frame {}", frame_index);
            //fs::rename(stream, "images")?;
            frame_index += 1;

        }
    }

    //fs::remove_dir("frames-from-video/");
    Ok(())
  }
}
