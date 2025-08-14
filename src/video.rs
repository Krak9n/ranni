use std::path::Path;
use std::fs;
use std::env;
use std::process::{Command, exit};

use image::ImageReader;
use image::{GenericImageView};

extern crate ffmpeg_next as ffmpeg;
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use ffmpeg::format;
use std::ffi::OsStr;

use walkdir::WalkDir;

pub struct VID {}

impl VID {

  /* VIDEO TO ASCII */
  // ---------------
  fn extract_frames(video_path: &str, output_dir: &str) {
      // Ensure output directory exists
      if !Path::new(output_dir).exists() {
          fs::create_dir_all(output_dir).unwrap();
      }

      let status = Command::new("ffmpeg")
          .arg("-i")
          .arg(video_path) 
          .arg(format!("{}/frame%04d.png", output_dir)) 
          .status()
          .expect("Failed to execute ffmpeg");

      if !status.success() {
          eprintln!("Error: ffmpeg command failed.");
          exit(1);
      }

      println!("Frames extracted successfully to {}", output_dir);
  }

  pub fn saving() {
    
    let args: Vec<String> = env::args()
    .collect();   
    
    let video_path = args[4].clone(); 
    let output_dir = "frames"; 

    Self::extract_frames(&video_path, output_dir);
    print!("\x1B[2J\x1B[1;1H");
  }
  
  fn get_str_ascii(intent: u8) -> &'static str {
      let index = intent / 32;
      let symbols = [" ","#","!","=","@","&","^","*"];
      return symbols[index as usize];
  }

  pub fn drawing(scale: u32) -> Result<(), Box<dyn std::error::Error>> {
    Self::saving(); 
    let args: Vec<String> = env::args()
      .collect();
       
    for entry in fs::read_dir("frames")? {
      // Part 3: get path and path str.
      let path = entry?.path();
      let path_str = path.to_str().unwrap();
      let reader = ImageReader::open(path_str).expect("FILE NOT FOUND");
     
      // smth happens here
      let dimensions = reader.into_dimensions()?;
      let (width, height) = dimensions;
      
      let img = image::open(path_str).unwrap();
      
      // make converting logic
      for y in 0..height {
        for x in 0..width {
          if y % (scale * 2) == 0 && x % scale == 0 {

            let pix = img.get_pixel(x,y);

            let mut intent = pix[0]/3 + pix[1]/3 + pix[2]/3;
                
              if pix[3] == 0 {
                intent = 0;
              }
              print!("{}", Self::get_str_ascii(intent));
            } 
        }
        if y % (scale * 2) == 0 {
          println!("");
        }
      }

      print!("\x1B[2J\x1B[1;1H");

    } 
    fs::remove_dir_all("frames/");

    Ok(())
  }

}
