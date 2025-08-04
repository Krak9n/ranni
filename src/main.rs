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

//#![feature(str_as_str)]
// user has to insert the full path
pub fn main() {

  // so, the command so far has to be smth like
  // ranni ~/file.txt
  // otherwise gonna be an error with index out of bonds
  let args: Vec<String> = env::args()
    .collect();
  
  bad_request(&args);

  // terminal size
  //terminal_get();

  let scale = args[2].parse::<u32>().unwrap();
  // printing func
  prints(scale);

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

  //Ok(())
}

fn get_str_ascii(intent :u8)-> &'static str {
    let index = intent/32;
    let symbols = [" ","!", "^", ".",",","-","~","+","=","@"];
    return symbols[index as usize];
}

// if printing doesnt containt filename
fn bad_request(args: &Vec<String>) {
  if args.len() < 1 {
    println!("bad usage.\nplease insert a filename after the executable\n");
    process::exit(3);
  }
}

fn get_image_dimensions(file_path: &str) -> Result<(u32, u32)> {
  let args: Vec<String> = env::args()
    .collect();

  let path = Path::new(&args[1]);
  let reader = Reader::open(path).expect("FILE NOT FOUND");
  let dimensions = reader.into_dimensions()?;
  Ok(dimensions)
}

pub fn prints(scale: u32) {
  let args: Vec<String> = env::args()
    .collect();

  match get_image_dimensions(&args[1]) {
    Ok((width, height)) => println!("dimensions: {} x {}", width, height),
    Err(e) => println!("error: {}", e),
  }

  let Ok((width, height)) = get_image_dimensions(&args[1]) else { todo!() };
  let img = image::open(&args[1]).unwrap();

  let (scrW, scrH) = termion::terminal_size().unwrap();
  // make converting logic
  for y in 0..height {
    for x in 0..width {
        if y % (scale * 2) == 0 && x % scale ==0{
            let pix = img.get_pixel(x,y);
            let mut intent = pix[0]/3 + pix[1]/3 + pix[2]/3;
            if pix[3] ==0{
                intent = 0;
            }
            print!("{}",get_str_ascii(intent));
        } 
    }
    if y%(scale*2)==0{
        println!("");
    }
  }

}
