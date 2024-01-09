use macroquad::{
    color::{BLACK, ORANGE},
    shapes::draw_rectangle,
    window::{clear_background, request_new_screen_size},
};

pub const PIXEL_SIZE: usize = 10;

pub fn render(pixels: [[bool; 64]; 32]) {
    request_new_screen_size((64 * PIXEL_SIZE) as f32, (32 * PIXEL_SIZE) as f32);

    clear_background(BLACK);

    for i in 0..32 {
        for j in 0..64 {
            if pixels[i][j] {
                draw_rectangle(
                    (j * PIXEL_SIZE) as f32,
                    (i * PIXEL_SIZE) as f32,
                    PIXEL_SIZE as f32,
                    PIXEL_SIZE as f32,
                    ORANGE,
                )
            }
        }
    }
}
