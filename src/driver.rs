use anyhow::anyhow;
use anyhow::Result;
use rpmlx90640::read_temperatures;
use rpmlx90640::TemperatureRead;

use linux_embedded_hal::I2cdev;
use mlx9064x::Mlx90640Driver;
use rpmlx90640::PIXEL_COUNT;

pub fn get_mlx90640_frame(bus: Option<&str>, address: Option<u8>) -> Result<TemperatureRead> {
    if let Some(bus) = bus {
        // try using the linux embedded hal driver

        if !bus.is_empty() {
            // default address is 0x33
            let addr = address.unwrap_or(0x33);

            let i2c_bus = I2cdev::new(bus)
                .map_err(|e| anyhow!("I2C Bus {} needs to be an I2C controller: {}", bus, e))?;

            let mut camera = Mlx90640Driver::new(i2c_bus, addr)?;

            let mut temperatures = [0f32; PIXEL_COUNT];
            camera
                .generate_image_if_ready(&mut temperatures)
                .map_err(|e| anyhow!("Error reading frame data from camera: {}", e))?;

            let min = temperatures
                .iter()
                .cloned()
                .fold(f32::INFINITY, |a, b| a.min(b));

            let max = temperatures
                .iter()
                .cloned()
                .fold(f32::INFINITY, |a, b| a.max(b));

            // wrap result in the convenient package of the rpmlx90640 driver
            return Ok(TemperatureRead {
                temperature_grid: temperatures,
                min_temp: min,
                max_temp: max,
            });
        }
    }

    // default to the RPI driver
    read_temperatures().map_err(|e| anyhow!("MLX90640 Error: {}", e))
}
