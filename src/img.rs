use std::path::Path;
use std::fs;
use std::env;

use image::ImageReader;
use image::{GenericImageView};

struct IMG {}

impl IMG {
  /* IMAGES TO ASCII */
  // ---------------
  fn get_image_dimensions(file_path: &str) -> Result<(u32, u32), E> {
    let args: Vec<String> = env::args()
      .collect();

    let path = Path::new(&args[1]);
    let reader = ImageReader::open(path).expect("FILE NOT FOUND");
    let dimensions = reader.into_dimensions()?;
    Ok(dimensions)
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
              print!("{}",get_str_ascii(intent));
          } 
      }
      if y % (scale * 2) == 0 {
          println!("");
      }
    }

    Ok(())
  }

}
