use std::io::BufWriter;
use std::io::Cursor;
use std::io::Write;

use anyhow::anyhow;
use anyhow::Result;

use clap::Parser;

use image::imageops::resize;
use image::imageops::FilterType;
use image::ImageBuffer;
use image::Rgb;

use itertools::Itertools;

use rpmlx90640::mlx_image::color_image;
use rpmlx90640::read_temperatures;
use rpmlx90640::TemperatureRead;
use rpmlx90640::PIXELS_HEIGHT;
use rpmlx90640::PIXELS_WIDTH;
use rpmlx90640::PIXEL_COUNT;

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

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

    /// Output filename; Defaults to no file output; Use `-` (dash) to output to stdout
    #[arg(short = 'o', long)]
    pub output: Option<String>,

    /// Text output in stderr as JSON
    #[arg(long, default_value = "false")]
    pub json: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    t_min: f32,
    t_max: f32,
    t_poi: Option<f32>,
}

fn main() {
    let opts = CliArgs::parse();

    match run(&opts) {
        Ok(stats) => {
            if opts.json {
                eprintln!(
                    "{}",
                    serde_json::to_string(&stats).unwrap_or("{}".to_string())
                );
            } else {
                let poi = if let Some(tpoi) = stats.t_poi {
                    format!(" Tpoi={}", tpoi)
                } else {
                    String::new()
                };

                eprintln!("Tmin={} Tmax={}{}", stats.t_min, stats.t_max, poi);
            }
        }
        Err(e) => {
            if opts.json {
                eprintln!("{}", json!({"error": e.to_string()}))
            } else {
                eprintln!("Error: {}", e)
            }
        }
    }
}

fn run(opts: &CliArgs) -> Result<Stats> {
    // get camera reading
    let camera_data = read_temperatures().map_err(|e| anyhow!("MLX90640 Error: {}", e))?;

    // start building result
    let mut stats = Stats {
        t_min: camera_data.min_temp,
        t_max: camera_data.max_temp,
        t_poi: None,
    };

    // hflip / vflip requested, apply early
    let camera_data = if opts.hflip || opts.vflip {
        let mut temperatures = camera_data.temperature_grid;

        if opts.hflip {
            horizontal_flip(&mut temperatures);
        }

        if opts.vflip {
            vertical_flip(&mut temperatures);
        }

        TemperatureRead {
            temperature_grid: temperatures,
            ..camera_data
        }
    } else {
        camera_data
    };

    let interpolation = opts.interpolation.unwrap_or(1);

    // there's a PoI defined and its raw (not scaled)
    if opts.poix.is_some() && opts.poiy.is_some() {
        let poix = opts.poix.unwrap_or(0);
        let poiy = opts.poiy.unwrap_or(0);

        let poi_index = if !opts.pois {
            poiy * PIXELS_WIDTH + poix
        } else {
            (poiy % interpolation) * PIXELS_WIDTH + (poix % interpolation)
        };
        stats.t_poi = camera_data.temperature_grid.get(poi_index).cloned();
    }

    if let Some(output_filename) = opts.output.as_deref() {
        let camera_image = color_image(&rpmlx90640::ColorTypes::Hue, &camera_data);

        let mut output_image = ImageBuffer::new(PIXELS_WIDTH as u32, PIXELS_HEIGHT as u32);

        for (pixel_index, (r, g, b)) in camera_image.pixels.iter().tuples().enumerate() {
            let y = pixel_index / PIXELS_WIDTH;
            let x = pixel_index % PIXELS_WIDTH;

            output_image.put_pixel(x as u32, y as u32, Rgb([*r, *g, *b]));
        }

        output_image = resize(
            &output_image,
            (PIXELS_WIDTH * opts.interpolation.unwrap_or(1)) as u32,
            (PIXELS_HEIGHT * opts.interpolation.unwrap_or(1)) as u32,
            FilterType::Lanczos3,
        );

        if output_filename == "-" {
            // output JPG to stdout
            let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
            output_image
                .write_to(&mut buffer, image::ImageFormat::Jpeg)
                .map_err(|e| anyhow!("Failed to write image resource buffer: {}", e))?;
            std::io::stdout()
                .write_all(
                    buffer
                        .into_inner()
                        .map_err(|e| anyhow!("Failed to flush image resource buffer: {}", e))?
                        .into_inner()
                        .as_slice(),
                )
                .map_err(|e| anyhow!("Failed to write image buffer to stdout: {}", e))?;
            std::io::stdout()
                .flush()
                .map_err(|e| anyhow!("Failed to flush stdout: {}", e))?;
        } else {
            // output to file
            output_image
                .save(output_filename)
                .map_err(|e| anyhow!("Failed to write image file: {}", e))?;
        }
    }

    Ok(stats)
}

fn horizontal_flip(temperatures: &mut [f32; PIXEL_COUNT]) {
    for y in 0..PIXELS_HEIGHT {
        let row_start = y * PIXELS_WIDTH;

        for x in 0..(PIXELS_WIDTH / 2) {
            let left_pixel = row_start + x;
            let right_pixel = row_start + (PIXELS_WIDTH - 1 - x);

            temperatures.swap(left_pixel, right_pixel);
        }
    }
}

fn vertical_flip(temperatures: &mut [f32; PIXEL_COUNT]) {
    for y in 0..(PIXELS_HEIGHT / 2) {
        let top_row = y * PIXELS_WIDTH;
        let bottom_row = (PIXELS_HEIGHT - 1 - y) * PIXELS_WIDTH;

        for x in 0..PIXELS_WIDTH {
            temperatures.swap(top_row + x, bottom_row + x);
        }
    }
}
