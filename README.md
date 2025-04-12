[![crates.io](https://img.shields.io/crates/v/mlx90640-capture?color=4d76ae)](https://crates.io/crates/mlx90640-capture)
[![dependency status](https://deps.rs/repo/github/iganev/mlx90640-capture/status.svg)](https://deps.rs/repo/github/iganev/mlx90640-capture)
[![build](https://github.com/iganev/mlx90640-capture/actions/workflows/rust.yml/badge.svg)](https://github.com/iganev/mlx90640-capture/actions/workflows/rust.yml)

# mlx90640-capture

Image capture utility for MLX90640 cameras.  
Works with Raspberry Pi boards as well as other Linux Embedded platforms with I2C support.  

## Installation

### 1. Get Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Compile crate
```bash
cargo install mlx90640-capture
```

## Usage

### I2C Bus and Device Address
By default uses a Raspberry Pi HAL based driver and looks for Raspberry Pi's default I2C bus and device address.  
Providing `--bus /dev/i2c-1` (and optionally `--address 0x33`) switches the camera driver to a Linux Embedded HAL based driver. Use this for any system other than Raspberry Pi. The optional `--address` argument accepts both decimal (51) and hex (0x33) values.  

### Interpolation
By default no image interpolation is applied. That produces an image with dimensions 32x24. Setting an interpolation value will multiple both the width and the height of the image preserving the aspect ratio. For example, `-i 10` will produce an image with dimensions 320x240.

### Color
The default color type is `hue`. Selecting a color type will change how the resulting image looks. The available options are: `hue`, `cheap` and `gray`.  

**Hue**: Colorful image with "colder" colors representing colder regions and "warmer" colors representing warmer regions.  
**Cheap**: Faster color image method using just blue (for cold regions), green (for medium temperatures) and red (for warmer regions).  
**Gray**: Grayscale image. Fastest method. Black for colder regions transitioning to white for warmer regions.

### Horizontal and Vertical flip

The two flags `--hflip` and `--vflip` allow the user to adjust the image in case the camera is mounted upside down. Can't really think of a scenario when you'd want to use just one of the flags, but for the sake of completeness - both are available.  

### Point of Interest

You can set a point of interest to be included in the `stderr` output as a temperature measurement.  

To set a point of interest you need to define its coordinates by passing `--poix <X>` and `--poiy <Y>`.  
To indicate that the coordinates are scaled up based on the interpolation scale factor, pass `--pois`.  
To add a visual indicator of the point of interest on the output image, pass `--poic`. The indicator resembles a crosshair surrounding the PoI. The crosshair takes 2 pixels on all sides of the PoI and inverts them for visual clarity and to prevent information loss.  

Note: Horizontal and vertical flips are applied **before** PoI probing.  

### Output

If no `-o` or `--output` argument is passed, no image will be generated.  
If `--output` is passed with value `-` then JPG image data will be written to `stdout`.  
If `--output` is passed with a valid filename and extension, the extension will be used to guess the file type and a corresponding image will be written to that file.  

Measurement results are output to `stderr` as plain text by default. Passing `--json` will format the measurements in JSON format. Any potential errors will also be wrapped in a JSON object and the error message will be placed in a property with name `error`.  

## Examples

Get temperature range in JSON format:  
```mlx90640-capture --json```

Get temperature range in text format, generate `test.jpg` and set a point of interest at the center.
```mlx90640-capture --poix 16 --poiy 12 --poic -o test.jpg```

Generate a horizontally and vertically flipped, 20x scaled image.
```mlx90640-capture -i 20 --hflip --vflip -o test.png```

Generate a horizontally and vertically flipped, 20x scaled image with point of interest in the center of the 1st quadrant and output the measurement results as JSON.  
```mlx90640-capture -i 20 --hflip --vflip --pois --poix 480 --poiy 120 --poic -o test.jpg --json```

Assuming you have ImageMagick installed. Generate a PNG image and pass it on to ImageMagick to make it semi-transparent.  
```mlx90640-capture -i 20 -c gray -o - | magick jpg:- -alpha set -background none -channel A -evaluate multiply 0.5 +channel test.png```

Get a 20x cheap color image on a different platform than a Raspberry Pi:
```mlx90640-capture -b /dev/i2c-1 -a 51 -i 20 -c cheap -o test.jpg```

Get a 10x image with center crosshair PoI on a different platform than a Raspberry Pi and JSON output:
```mlx90640-capture --json -b /dev/i2c-1 -a 0x33 -i 10 --poix 16 --poiy 12 --poic -o test.jpg```
## Help

```
mlx90640-capture -h
Image capture utility for MLX90640 cameras

Usage: mlx90640-capture [OPTIONS]

Options:
  -b, --bus <BUS>                      I2C Bus (switches to Linux Embedded HAL driver)
  -a, --address <ADDRESS>              I2C Address; Works only when bus is selected; defaults to 0x33
  -i, --interpolation <INTERPOLATION>  Interpolation scale factor
  -c, --color <COLOR>                  Image color type: hue, gray, cheap; default: hue
      --hflip                          Horizontal flip
      --vflip                          Vertical flip
      --pois                           Point of interest coordinates are scaled up, instead of raw
  -x, --poix <POIX>                    Point of interest X coordinate
  -y, --poiy <POIY>                    Point of interest Y coordinate
      --poic                           Draw point of interest crosshair on output image
  -o, --output <OUTPUT>                Output filename; Defaults to no file output; Use `-` (dash) to output to stdout
      --json                           Text output in stderr as JSON
  -h, --help                           Print help
  -V, --version                        Print version

```

## Acknowledgements

Thanks to [rpmlx90640](https://crates.io/crates/rpmlx90640) and [mlx9064x](https://crates.io/crates/mlx9064x) crates that provide the driver backends for Raspberry Pi and Linux Embedded HAL.

# License

This project is open sourced under the MIT License.