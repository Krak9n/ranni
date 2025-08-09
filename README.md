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
![](https://github.com/Krak9n/ranni/blob/main/gif/animation.gif)

**Usage** <br />
Currently supports only image formats provided by [image](https://docs.rs/image/latest/image/index.html)

To execute just run:
`ranni ~/images/photo.png 6` <br />
in which second argument is the path to photo, and the third is the scale of printing.

**Small Roadmap**
+ [] RGBA colors 
+ [] video to ascii
+ [] saving option
+ [x] more user friendly options for input

**Used as reference**
+ [this](https://github.com/BrendanBetterman/Rust-Ascii-Art-Generator) amazing tutorial
