// every time when in development needed 
// to change args amount on +1.

#[warn(unused_imports)]
use std::fs;
use std::fs::File;
use std::env;
use std::error::Error;
use std::path::Path;
use std::process;
//use std::error::Error;
use anyhow::Result;

use opencv::videoio;
use opencv::videoio::*;
use opencv::videoio::{VideoCapture};
use opencv::prelude::VideoCaptureTraitConst;
use opencv::core::Size_;
use opencv::imgcodecs::imread;
use opencv::prelude::MatTraitConst;
use opencv::core::Mat;
use opencv::core::Vec3b;

//#![feature(str_as_str)]
// user has to insert the full path
pub fn main() -> Result<(), Box<dyn std::error::Error>>{

  // so, the command so far has to be smth like
  // ranni ~/file.txt
  // otherwise gonna be an error with index out of bonds
  let args: Vec<String> = env::args()
    .collect();
  
  bad_request(&args);

  // terminal size
  //terminal_get();

  /* FOR VIDEO */
  video_inf(&args);
  
  //let scale = args[2].parse::<i32>().unwrap();
  
  /* FOR IMAGES */
  // printing func
  //prints(scale);

  /*
  let mut file = File::create(&args[3].clone());
  file.write_all(prints(scale)).except("failed to write");

  // printout the result 
  // after saving the file
  if Path::new(&file).exists() {
      match fs::read_to_string(file) {
          Ok(content) => println!("{}", content),
          Err(err) => eprintln!("error reading file: {}", err),
      }
  } else {
      eprintln!("file does not exist: {}", file_path);
  }
  */

  Ok(())
}

fn get_str_ascii(intent: u8)-> &'static str {
    let index = intent/32;
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

pub fn prints(scale: i32) -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = env::args()
    .collect();
   
  let path: &str = &args[1].clone();
  let img = imread(path, opencv::imgcodecs::IMREAD_COLOR)?;
  let size = img.size()?;

  let width = size.width;
  let height = size.height;
  
  // make converting logic
  for y in 0..height {
    for x in 0..width {
        if y % (scale * 2) == 0 && x % scale == 0 {
            let pix = *img.at_2d::<Vec3b>(y, x)?; // return if error
            
            let mut intent = ((pix[0] as u16 + pix[1] as u16 + pix[2] as u16) / 3).try_into().unwrap();
            
            // problem with intensity. 
            // cannot make it working
            // each time i store pixels in Vec4 
            // program stops working
            /*
            if pix[3] ==0{
                intent = 0;
            }
            */
            
            print!("{}", get_str_ascii(intent));
        } 
    }
    if y % (scale*2) == 0 {
        println!("");
    }
  }
  Ok(())
}

pub fn video_inf(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
  
  let path: &str = &args[1].clone();
  let mut video = videoio::VideoCapture::from_file(path, videoio::CAP_ANY)?;
  
  if !video.is_opened()? {
    panic!("unable to open default camera!");
  }

  //fs::remove_dir("./images/")?;

  let width = video.get(CAP_PROP_FRAME_WIDTH)?.round() as i32;
  let height = video.get(CAP_PROP_FRAME_HEIGHT)?.round() as i32;
  let frames = video.get(CAP_PROP_FRAME_COUNT)?.round();

  let size = width as i32 + height as i32;

  println!("frame width: {}", video.get(CAP_PROP_FRAME_WIDTH)?.round()); //Frame width:1280
  println!("frame height: {}", video.get(CAP_PROP_FRAME_HEIGHT)?.round()); //Frame height720
  println!("fps: {}", video.get(CAP_PROP_FRAME_COUNT)?.round()); //Frame height720  // at the end

  let fourcc = videoio::VideoWriter::fourcc('m', 'p', '4', 'v')?; // mpeg4 codec
  //let fourcc = videoio::VideoWriter::fourcc('X', '2', '6', '4')?; //m1 x264=>H.264
  //let fourcc = video.get(CAP_PROP_FOURCC)?.round();
  

  // try to implement later smth like saving depending
  // on the args last name argument
  // same with the codec
  fs::create_dir("./images");
  let mut out = VideoWriter::new_def("./images/output.mp4", fourcc, frames, Size_::new(width, height));
  
  let current = 0;


  fs::remove_dir("./images/")?;

  video.release()?;
  Ok(())
}
