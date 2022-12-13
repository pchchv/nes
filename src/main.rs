pub mod cpu;
pub mod opcodes;
use sdl2::pixels::Color;

#[macro_use]
extern crate lazy_static;

fn color(byte: u8) -> Color {
    match byte {
        0 => Color::BLACK,
        1 => Color::WHITE,
        2 | 9 => Color::GREY,
        3 | 10 => Color::RED,
        4 | 11 => Color::GREEN,
        5 | 12 => Color::BLUE,
        6 | 13 => Color::MAGENTA,
        7 | 14 => Color::YELLOW,
        _ => sdl2::pixels::Color::CYAN,
    }
}

fn main() {
    println!("Hello, world!");
}
