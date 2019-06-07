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

/// Rotating square
fn sketch_03(app: &nannou::App, frame: nannou::Frame) -> nannou::Frame {
    let draw = app.draw();
    let window_rect = app.window_rect();
    draw.background().color(nannou::color::BLACK);

    draw.rect()
        .color(nannou::color::LIGHT_ORANGE)
        .width(window_rect.w() / 3.0)
        .height(window_rect.w() / 3.0)
        .rotate((app.duration.since_start.as_millis() as f32 / 500.0) / 3.14)
        .x(0.0)
        .y(0.0);

    draw.to_frame(app, &frame).unwrap();

    frame
}

fn main() {
    nannou::sketch(sketch_03);
}
