#![allow(dead_code)]

use nannou::prelude::*;

fn sketch_01(_app: &nannou::App, frame: nannou::Frame) -> nannou::Frame {
    frame.clear(nannou::color::DARK_RED);
    frame
}

/// Black triangle on white background.
fn sketch_02(app: &nannou::App, frame: nannou::Frame) -> nannou::Frame {
    let draw = app.draw();
    let window_rect = app.window_rect();
    draw.background().color(nannou::color::WHITE);
    draw.tri()
        .points(
            window_rect.top_left(),
            (0.0, window_rect.bottom_left().y).into(),
            window_rect.top_right(),
        )
        .color(nannou::color::BLACK);
    draw.to_frame(app, &frame).unwrap();
    frame
}

fn main() {
    nannou::sketch(sketch_02);
}
