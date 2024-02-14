pub use macroquad::prelude::*;
pub extern crate rand;
pub use rand::prelude::*;

pub fn draw_text_centered(text: &str, x: f32, y:f32, font_size: f32, color: Color, font: Option<&Font>) {
    let dimensions = measure_text(text, font, font_size as u16, 1.0);

    draw_text(text, x-dimensions.width/2., y+dimensions.height/2., font_size, color);
}

pub fn get_endnum() -> i32 {
    let mut x;
    loop {
        x = rand::thread_rng().gen_range(-300i32..301i32);
        if (-30..31).contains(&x) {
            continue
        }
        break
    }
    x
}