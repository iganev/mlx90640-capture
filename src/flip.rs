use rpmlx90640::{PIXELS_HEIGHT, PIXELS_WIDTH, PIXEL_COUNT};

pub fn horizontal_flip(temperatures: &mut [f32; PIXEL_COUNT]) {
    for y in 0..PIXELS_HEIGHT {
        let row_start = y * PIXELS_WIDTH;

        for x in 0..(PIXELS_WIDTH / 2) {
            let left_pixel = row_start + x;
            let right_pixel = row_start + (PIXELS_WIDTH - 1 - x);

            temperatures.swap(left_pixel, right_pixel);
        }
    }
}

pub fn vertical_flip(temperatures: &mut [f32; PIXEL_COUNT]) {
    for y in 0..(PIXELS_HEIGHT / 2) {
        let top_row = y * PIXELS_WIDTH;
        let bottom_row = (PIXELS_HEIGHT - 1 - y) * PIXELS_WIDTH;

        for x in 0..PIXELS_WIDTH {
            temperatures.swap(top_row + x, bottom_row + x);
        }
    }
}
