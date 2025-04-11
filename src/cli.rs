use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Interpolation scale factor
    #[arg(short = 'i', long)]
    pub interpolation: Option<usize>,

    /// Horizontal flip
    #[arg(long, default_value = "false")]
    pub hflip: bool,

    /// Vertical flip
    #[arg(long, default_value = "false")]
    pub vflip: bool,

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

    /// Output filename; Defaults to no file output; Use `-` (dash) to output to stdout
    #[arg(short = 'o', long)]
    pub output: Option<String>,

    /// Text output in stderr as JSON
    #[arg(long, default_value = "false")]
    pub json: bool,
}
