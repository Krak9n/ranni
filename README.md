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

**!Deprecated video example!**
To execute just run:
`ranni -i ~/images/photo.png -s 6` <br />
in which third argument is the path to image/video, and the fourth is the scale of printing. <br />

**Small Roadmap**
+ [] RGBA colors 
+ [] video to ascii
+ [x] more user friendly options for input

**If you want to save it for later**
Just add standard flag `> filename.format`
Example:
```
ranni -i ~/Downloads/love.png -s 6 > save.txt
```

**Used as reference**
+ [this](https://github.com/BrendanBetterman/Rust-Ascii-Art-Generator) amazing tutorial
