use clap::Parser;
use clap_num::maybe_hex;
use rpmlx90640::ColorTypes;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    // Device params
    /// I2C Bus (switches to Linux Embedded HAL driver)
    #[arg(short = 'b', long)]
    pub bus: Option<String>,

    /// I2C Address; Works only when bus is selected; defaults to 0x33
    #[arg(short = 'a', long, value_parser=maybe_hex::<u8>)]
    pub address: Option<u8>,

    // Processing params
    /// Interpolation scale factor
    #[arg(short = 'i', long, value_parser = clap::value_parser!(u16).range(1..100))]
    pub interpolation: Option<u16>,

    /// Image color type: hue, gray, cheap; default: hue
    #[arg(short = 'c', long)]
    pub color: Option<ColorTypes>,

    /// Horizontal flip
    #[arg(long, default_value = "false")]
    pub hflip: bool,

    /// Vertical flip
    #[arg(long, default_value = "false")]
    pub vflip: bool,

    // PoI params
    /// Point of interest coordinates are scaled up, instead of raw
    #[arg(long, default_value = "false")]
    pub pois: bool,

    /// Point of interest X coordinate
    #[arg(short = 'x', long)]
    pub poix: Option<usize>,

    /// Point of interest Y coordinate
    #[arg(short = 'y', long)]
    pub poiy: Option<usize>,

    /// Draw point of interest crosshair on output image
    #[arg(long, default_value = "false")]
    pub poic: bool,

    // Output params
    /// Output filename; Defaults to no file output; Use `-` (dash) to output to stdout
    #[arg(short = 'o', long)]
    pub output: Option<String>,

    /// Text output in stderr as JSON
    #[arg(long, default_value = "false")]
    pub json: bool,
}
