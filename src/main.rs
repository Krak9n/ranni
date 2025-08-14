#[warn(unused_imports)]
use std::env;
use std::process;
//use std::error::Error;
use anyhow::Result;
use clap::Parser;

pub mod img;
pub mod video;

// for better command line use
// last one on the roadmap
#[derive(Parser)]
struct Args {
  // tell if it's a video
  // or an image
  #[arg(short = 't', long = "type")]
  t: String,
  // input file
  #[arg(short = 'i', long = "input")]
  i: String,
  // scaling
  #[arg(short = 's', long = "scale")]
  s: String,
}

//#![feature(str_as_str)]
// user has to insert the full path
pub fn main() -> Result<(), Box<dyn std::error::Error>> {

  let args: Vec<String> = env::args()
    .collect();
  bad_request(&args);

  let input = Args::parse();
  let scale = args[6].parse::<u32>().unwrap();

  if input.t == "image" {
    img::IMG::prints(scale);
  }

  if input.t == "video" {
    video::VID::drawing(scale);
  } 

  Ok(())
}

// if printing doesnt containt filename
fn bad_request(args: &Vec<String>) {
  if args.len() < 2 {
    print!("bad usage.\nplease insert a filename after the executable\n");
    process::exit(3);
  }
}

