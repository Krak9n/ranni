// every time when in development needed 
// to change args amount on +1.

#[warn(unused_imports)]
use std::env;
use std::process;
//use std::error::Error;
use anyhow::Result;

pub mod img;
//pub mod video;
//pub mod rgba;

// for better command line use
// last one on the roadmap
struct Args {
  // input file
  i: String,
  // scaling
  scale: u32,
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

  //let path = &args[1].clone();
  //video_to_ascii(path);

  // printing func
  let scale = args[2].parse::<u32>().unwrap();
  img::IMG::prints(scale);

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


