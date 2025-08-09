use std::path::Path;
use std::error::Error;
use std::env;

use image::ImageReader;
use image::{GenericImageView};

pub struct IMG<'a> {
  background: &'a str,
}

impl IMG<'_> {
  /* IMAGES TO ASCII */
  // ---------------
  fn get_image_dimensions(file_path: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let args: Vec<String> = env::args()
      .collect();

    let path = Path::new(&args[1]);
    let reader = ImageReader::open(path).expect("FILE NOT FOUND");
    let dimensions = reader.into_dimensions()?;
    Ok(dimensions)
  }

  fn get_str_ascii(intent :u8)-> &'static str {
      let index = intent / 32;
      let symbols = [" ","!", "^", ".",",","-","~","+","=","@"];
      return symbols[index as usize];
  }

  pub fn prints(scale: u32) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args()
      .collect();

    let Ok((width, height)) = Self::get_image_dimensions(&args[1]) else { todo!() };
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
              print!("{}", Self::get_str_ascii(intent));
          } 
      }
      if y % (scale * 2) == 0 {
          println!("");
      }
    }

    Ok(())
  }

  fn get_to_rgba() {
    let rgb = IMG {
      background: "200200200",
    };
    
    

  }

}
