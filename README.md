**Needed libraries**
+ cargo
+ rustc

After installing both, just run `install.sh` by:
```
git clone https://github.com/Krak9n/ranni.git   
cd ranni/
chmod +x install.sh
./install
```

**Example** <br />
![](https://github.com/Krak9n/ranni/blob/main/gif/output.gif)

**Usage** <br />
Supports only image formats provided by [image](https://docs.rs/image/latest/image/index.html), 
and video format by [ffmpeg_next](https://docs.rs/ffmpeg-next/latest/ffmpeg_next/index.html).

To execute just run:
`ranni -t image -i ~/images/photo.png -s 6` <br />
in which first argument is type of an input(image or video), <br />
second is the path to image/video, <br />
and the third is the scale of printing. <br />

**Small Roadmap**
+ >[] RGBA colors **(couldn't figure out)** 
+ [x] video to ascii 
+ [x] more user friendly options for input

**If you want to save it for later**
Just add standard flag `> filename.format`
Example:
```
ranni -t image -i ~/Downloads/love.png -s 6 > save.txt
```

**Used as reference**
+ [this](https://github.com/BrendanBetterman/Rust-Ascii-Art-Generator) amazing tutorial
