Small utility for converting images and videos into ASCII.

### Dependencies
* cargo

### Installation
```bash
$ cd ranni
$ cargo install --path .
```

### Example 
![](https://github.com/Krak9n/ranni/blob/main/example/output.gif)

### Usage
Supports only image formats provided by [image](https://docs.rs/image/latest/image/index.html), 
and video format by [ffmpeg_next](https://docs.rs/ffmpeg-next/latest/ffmpeg_next/index.html).

### Executing
```bash
$ ranni -t image -i ./example/sakura.jpg -s 6
  -t: type of the input  
  -i: input path  
  -s: the scale of printing  
```

### Roadmap
+ [x] RGBA colors **(couldn't figure out)** 
+ [x] video to ascii 
+ [x] more user friendly options for input
+ [] refactor
+ [] add clippy support
+ [] fix video processing

### Used as reference
+ [this amazing tutorial](https://github.com/BrendanBetterman/Rust-Ascii-Art-Generator)
