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

  // so, the command so far has to be smth like
  // ranni ~/file.txt
  // otherwise gonna be an error with index out of bonds
  let args: Vec<String> = env::args()
    .collect();
  bad_request(&args);

  // input has to be indicate: -i path/file.format
  /* HERE MUST ADD SOME SORT OF SWITCH FOR SCANNING WITH FFMPEG
   * FILE FORMATS */
  // scale has to indicate size: -s number

  let input = Args::parse();
  
  // -i = 1 // to code
  // path = 2
  // -s = 3 // to code
  // scale = 4

  let scale = args[4].parse::<u32>().unwrap();
  
 // video::VID::saving();
  //video::VID::drawing(scale);
  img::IMG::prints(scale);

  Ok(())
}

// if printing doesnt containt filename
fn bad_request(args: &Vec<String>) {
  if args.len() < 2 {
    print!("bad usage.\nplease insert a filename after the executable\n");
    process::exit(3);
  }
}

