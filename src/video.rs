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
          .arg("-vf")
          .arg("fps=1") 
          .arg("-q:v")
          .arg("2") 
          .arg(format!("{}/frame%04d.jpg", output_dir)) 
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
    
    let video_path = args[2].clone(); 
    let output_dir = "frames"; 

    Self::extract_frames(&video_path, output_dir);
    print!("\x1B[2J\x1B[1;1H");
  }
  
  fn get_str_ascii(intent: u8) -> &'static str {
      let index = intent / 32;
      let symbols = [" ","!", "^", ".",",","-","~","+","=","@"];
      return symbols[index as usize];
  }

  pub fn drawing(scale: u32) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args()
      .collect();
    
    for entry in WalkDir::new("frames") {
      let entry = entry.unwrap();

      let file: &str = entry.file_name().to_str().unwrap();
      let path = Path::new(file);
      let reader = ImageReader::open(path).expect("FILE NOT FOUND");
      let dimensions = reader.into_dimensions()?;

      let (width, height) = dimensions;
      let img = image::open(&args[2]).unwrap();

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
    }
    fs::remove_dir("frames");

    Ok(())

  }
}
