use anyhow::anyhow;
use anyhow::Result;
use image::imageops::resize;
use image::imageops::FilterType;
use image::ImageBuffer;
use image::Rgb;
use itertools::Itertools;
use rpmlx90640::take_image;
use rpmlx90640::PIXELS_HEIGHT;
use rpmlx90640::PIXELS_WIDTH;

/// scale up the width and height of the final image
pub const INTERPOLATION: usize = 25;

fn main() -> Result<()> {
    let camera_image =
        take_image(&rpmlx90640::ColorTypes::Hue).map_err(|e| anyhow!("MLX90640 Error: {}", e))?;

    let mut output_image = ImageBuffer::new(PIXELS_WIDTH as u32, PIXELS_HEIGHT as u32);

    for (pixel_index, (r, g, b)) in camera_image.pixels.iter().tuples().enumerate() {
        let y = pixel_index / PIXELS_WIDTH;
        let x = pixel_index % PIXELS_WIDTH;

        output_image.put_pixel(x as u32, y as u32, Rgb([*r, *g, *b]));
    }

    output_image = resize(
        &output_image,
        (PIXELS_WIDTH * INTERPOLATION) as u32,
        (PIXELS_HEIGHT * INTERPOLATION) as u32,
        FilterType::Lanczos3,
    );

    output_image.save("output.jpg")?;

    println!(
        "Tmin={} Tmax={}",
        camera_image.temperature_read.min_temp, camera_image.temperature_read.max_temp
    );

    Ok(())
}
