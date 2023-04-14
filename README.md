# Stargazer
![mandelbrot](https://user-images.githubusercontent.com/24441325/232087718-80ead6b4-b8ab-4e8d-a0da-6ae0724b1828.png)
## A mandelbrot fractal explorer with export functionality

Stargazer is a program written to allow easy and fast exploration of the mandelbrot set in a resolution that allows for 
quick exploration and then allows the user to export a nice higher resolution version of the image they saw in low resolution in the program.

It's also a excercise in optimizing programs. I hope to make it as fast as possible, it's currently fairly fast but not fast enough.

## How to run
### For an unoptimized slower experience
  cargo run
### For a smoother and faster experience
  cargo build --release
  ./target/release/stargazer

## How to use
- WASD for navigating
- Q and E for zooming
- R to render a high resolution image
