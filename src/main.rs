pub(crate) mod cli;
pub(crate) mod driver;
pub(crate) mod flip;

use std::io::BufWriter;
use std::io::Cursor;
use std::io::Write;

use anyhow::anyhow;
use anyhow::Result;

use clap::Parser;

use cli::CliArgs;
use driver::get_mlx90640_frame;
use flip::horizontal_flip;
use flip::vertical_flip;
use image::imageops::resize;
use image::imageops::FilterType;
use image::ImageBuffer;
use image::Rgb;

use itertools::Itertools;

use rpmlx90640::mlx_image::color_image;
use rpmlx90640::TemperatureRead;
use rpmlx90640::PIXELS_HEIGHT;
use rpmlx90640::PIXELS_WIDTH;

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

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
    let camera_data = get_mlx90640_frame(None, None)?;

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

    if interpolation < 1 {
        return Err(anyhow!("Interpolation cannot be less than 1"));
    }

    // there's a PoI defined and its raw (not scaled)
    if opts.poix.is_some() && opts.poiy.is_some() {
        let poix = opts.poix.unwrap_or(0);
        let poiy = opts.poiy.unwrap_or(0);

        let poi_index = if !opts.pois {
            poiy * PIXELS_WIDTH + poix
        } else {
            (poiy / interpolation) * PIXELS_WIDTH + (poix / interpolation)
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

        // draw crosshair
        if opts.poic {
            let poix = if opts.pois {
                opts.poix.unwrap_or(0)
            } else {
                opts.poix.unwrap_or(0) * interpolation
            };
            let poiy = if opts.pois {
                opts.poiy.unwrap_or(0)
            } else {
                opts.poiy.unwrap_or(0) * interpolation
            };

            // (x-1),y; (x-2),y; (x+1),y, (x+2),y; x,(y-1); x,(y-2); x,(y+1); x,(y+2);
            let crosshair_pixels: Vec<(i64, i64)> = vec![
                ((poix - 1) as i64, poiy as i64),
                ((poix - 2) as i64, poiy as i64),
                ((poix + 1) as i64, poiy as i64),
                ((poix + 2) as i64, poiy as i64),
                (poix as i64, (poiy - 1) as i64),
                (poix as i64, (poiy - 2) as i64),
                (poix as i64, (poiy + 1) as i64),
                (poix as i64, (poiy + 2) as i64),
            ];

            for (x, y) in crosshair_pixels {
                if x < 0
                    || y < 0
                    || x > output_image.width() as i64
                    || y > output_image.height() as i64
                {
                    continue;
                }

                if let Some(pixel) = output_image.get_pixel_mut_checked(x as u32, y as u32) {
                    *pixel = [255u8 - pixel.0[0], 255u8 - pixel.0[1], 255u8 - pixel.0[2]].into();
                }
            }
        }

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
