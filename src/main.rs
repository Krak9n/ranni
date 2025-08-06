// every time when in development needed 
// to change args amount on +1.

#[warn(unused_imports)]
use std::fs;
use std::fs::File;
use std::env;
use std::path::Path;
use std::process;
//use std::error::Error;
use anyhow::Result;

use image::io::Reader;
use image::{GenericImageView};

extern crate ffmpeg_next as ffmpeg;
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use ffmpeg::format;

struct Args {
  // input file
  i: String,
  // scaling
  scale: u32,
}

//#![feature(str_as_str)]
// user has to insert the full path
pub fn main() -> Result<(), ffmpeg::Error> {

  // so, the command so far has to be smth like
  // ranni ~/file.txt
  // otherwise gonna be an error with index out of bonds
  let args: Vec<String> = env::args()
    .collect();
  
  bad_request(&args);

  //let path = &args[1].clone();
  //video_to_ascii(path);

  // printing func
  let scale = args[2].parse::<u32>().unwrap();
  prints(scale);

  Ok(())
}

fn get_str_ascii(intent :u8)-> &'static str {
    let index = intent / 32;
    let symbols = [" ","!", "^", ".",",","-","~","+","=","@"];
    return symbols[index as usize];
}

// if printing doesnt containt filename
fn bad_request(args: &Vec<String>) {
  if args.len() < 2 {
    println!("bad usage.\nplease insert a filename after the executable\n");
    process::exit(3);
  }
}

/* IMAGES TO ASCII */
// ---------------
fn get_image_dimensions(file_path: &str) -> Result<(u32, u32)> {
  let args: Vec<String> = env::args()
    .collect();

  let path = Path::new(&args[1]);
  let reader = Reader::open(path).expect("FILE NOT FOUND");
  let dimensions = reader.into_dimensions()?;
  Ok(dimensions)
}

pub fn prints(scale: u32) -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = env::args()
    .collect();

  let Ok((width, height)) = get_image_dimensions(&args[1]) else { todo!() };
  let img = image::open(&args[1]).unwrap();

  // make converting logic
  for y in 0..height {
    for x in 0..width {
        if y % (scale * 2) == 0 && x % scale == 0 {

            let pix = img.get_pixel(x,y);

            let mut intent = pix[0]/3 + pix[1]/3 + pix[2]/3;
            
            if pix[3] == 0 {
                intent = 0;
            }
            print!("{}",get_str_ascii(intent));
        } 
    }
    if y % (scale * 2) == 0 {
        println!("");
    }
  }
  Ok(())

}
// ------------------

/* VIDEO TO ASCII */
// ---------------
pub fn video_to_ascii(path: &str) -> Result<()> {
   let args: Vec<String> = env::args()
    .collect();

  ffmpeg::init().unwrap();
  fs::create_dir("./images");

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

  //fs::remove_dir("images/");
  Ok(())
}
