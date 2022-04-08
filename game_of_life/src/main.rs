use piston_window::{PistonWindow, WindowSettings, rectangle};

extern crate rand;
extern crate piston_window;
extern crate array2d;

mod game;
mod draw;

use piston_window::*;
use piston_window::types::Color;
use crate::draw::to_coord_u32;

fn main () {
    let (width, height): (i32, i32) = (50, 50);

    let mut window: PistonWindow = WindowSettings::new(
        "Game of Life",
        [to_coord_u32(width), to_coord_u32(height)],
    ).exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 100.0, 100.0],
            c.transform,
            g)
        });
    }
}
