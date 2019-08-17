use nannou::noise::NoiseFn;
use nannou::prelude::*;

#[derive(Debug)]
struct Model {
    bubble: Vec<Point2>,
    radiuses: Vec<f64>,
    noise: nannou::noise::Perlin,
}

impl Model {
    fn bg_color() -> Rgba {
        nannou::color::BLUE
    }

    fn new(app: &App) -> Self {
        app.new_window().build().unwrap();

        Model {
            bubble: Vec::with_capacity(360),
            noise: nannou::noise::Perlin::new(),
            radiuses: Vec::with_capacity(300),
        }
    }

    fn view(app: &App, model: &Model, frame: Frame) -> Frame {
        frame.clear(Self::bg_color());

        let draw = app.draw();

        for (origin, next) in model.bubble.iter().zip(model.bubble.iter().skip(1)) {
            draw.line().start(*origin).end(*next);
        }

        draw.to_frame(app, &frame).unwrap();

        frame
    }

    fn update(app: &App, model: &mut Model, update: nannou::event::Update) {
        model.bubble.clear();
        model.radiuses.clear();

        let duration_factor = (app.duration.since_start.as_millis() as f64 / 1000.0).sin();
        let initial = model.noise.get([duration_factor, 0.0]) * 50.0;
        model.radiuses.push(initial);

        for angle in 0..i32::max_value() {
            let angle_radians = (angle as f32 / 360.0) * (2.0 * PI);

            let radius = model.noise.get([duration_factor, angle_radians as f64]) * 50.0;
            model.radiuses.push(radius);

            if angle > 360 && radius.floor() == initial.floor() {
                break;
            }
        }

        dbg!(&model);
        for (idx, radius) in model.radiuses.iter().enumerate() {
            let angle = (idx as f32 / model.radiuses.len() as f32) * TAU;
            model
                .bubble
                .push(polar_to_cartesian(angle, *radius as f32 + 200.0))
        }

        // for angle in 0..=360 {
        //     let angle_radians = (angle as f32 / 360.0) * (2.0 * PI);
        //     let noise = dbg!(model.noise.get([duration_factor, angle_radians as f64]));
        //     let radius: f32 = noise as f32 * 50.0 + 200.0;
        //     let point = Point2 {};
        //     model.bubble.push(point);
        // }
    }
}

fn polar_to_cartesian(angle: f32, radius: f32) -> Point2 {
    Point2 {
        x: radius * angle.cos(),
        y: radius * angle.sin(),
    }
}

fn main() -> Result<(), failure::Error> {
    nannou::app(Model::new)
        .view(Model::view)
        .update(Model::update)
        .run();

    Ok(())
}
