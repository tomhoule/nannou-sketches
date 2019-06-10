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

struct App01Model {
    points: Vec<(u32, u32)>,
}

impl App01Model {
    /// In pixels.
    const SIDE_LENGTH: u32 = 14;

    fn new(app: &App) -> App01Model {
        let (w, h) = (1000, 600);
        let mut points = Vec::with_capacity(
            (w / Self::SIDE_LENGTH + 1) as usize * (h / Self::SIDE_LENGTH + 1) as usize,
        );

        let window = app
            .new_window()
            .with_dimensions(w, h)
            .view(Self::view)
            .build()
            .unwrap();

        let mut cursor: (u32, u32) = (0, 0);
        let bottom_right: (u32, u32) = app.main_window().inner_size_pixels();

        let finished_drawing = |cursor: (u32, u32)| cursor.1 > bottom_right.1;

        let mut row = 0;

        while !finished_drawing(cursor) {
            points.push(cursor);

            if cursor.0 > bottom_right.0 {
                // Next line
                row += 1;
                cursor = (
                    if row % 3 == 0 { Self::SIDE_LENGTH } else { 0 },
                    cursor.1 + Self::SIDE_LENGTH,
                );
            } else {
                let narrow = row % 3 == 0;
                // Next point
                cursor = (cursor.0 + Self::SIDE_LENGTH * 2, cursor.1);
            }
        }

        App01Model { points }
    }

    fn view(app: &App, model: &Self, frame: Frame) -> Frame {
        frame.clear(nannou::color::WHITE);

        let window_rect = app.window_rect();
        let top_left = window_rect.top_left();

        let draw = app.draw();

        for point in &model.points {
            draw.ellipse()
                .radius(4.0)
                .color(nannou::color::RED)
                .x(top_left.x + point.0 as f32)
                .y(top_left.y - point.1 as f32)
                // .w(20.0)
                // .h(20.0)
                .finish()
                .unwrap();
        }

        draw.to_frame(app, &frame).unwrap();

        frame
    }
}

fn app_01() {
    nannou::app(App01Model::new).run();
}

fn main() {
    nannou::sketch(sketch_04);
    // app_01()
}

fn sketch_04(app: &nannou::App, frame: nannou::Frame) -> nannou::Frame {
    let draw = app.draw();
    frame.clear(nannou::color::WHITE);

    const SIDE: f32 = 16.0;
    const GAP: f32 = 2.0;

    let window_rect = app.window_rect();

    let mut cursor = window_rect.top_left();

    while cursor.y > window_rect.bottom_right().y {
        let points: Vec<Point2> = vec![
            // Top left
            cursor,
            // Top right
            Point2 {
                x: cursor.x + SIDE,
                y: cursor.y,
            },
            // Higher-middle right
            Point2 {
                x: cursor.x + (SIDE * 1.5),
                y: cursor.y - (SIDE * 0.5),
            },
            // Lower-middle right
            Point2 {
                x: cursor.x + (SIDE * 1.5),
                y: cursor.y - (SIDE * 1.5),
            },
            // Bottom right
            Point2 {
                x: cursor.x + SIDE,
                y: cursor.y - (SIDE * 2.0),
            },
            // Bottom left
            Point2 {
                x: cursor.x,
                y: cursor.y - (SIDE * 2.0),
            },
            Point2 {
                x: cursor.x - (SIDE * 0.5),
                y: cursor.y - (SIDE * 1.5),
            },
            Point2 {
                x: cursor.x - (SIDE * 0.5),
                y: cursor.y - (SIDE * 0.5),
            },
        ];
        draw.polygon()
            .points(points)
            .color(nannou::color::RED)
            .finish()
            .unwrap();

        cursor.x += (SIDE * 2.0) + GAP;

        if cursor.x > window_rect.top_right().x {
            cursor.y -= (SIDE * 2.0) + GAP;
            cursor.x = window_rect.top_left().x;
        }
    }

    draw.to_frame(app, &frame).unwrap();

    frame
}
