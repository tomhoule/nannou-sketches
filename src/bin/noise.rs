use nannou::prelude::*;
struct Model {
    points: Vec<Point2>,
}

impl Model {
    fn new(app: &App) -> Self {
        app.new_window().build().unwrap();

        Model {
            points: Vec::with_capacity(20_000),
        }
    }

    fn update(app: &App, model: &mut Self, update: Update) {
        use nannou::noise::NoiseFn;
        let x =
            app.duration.since_start.as_millis() as f32 / 10.0 - app.window_rect().w() as f32 / 2.0;
        let y = nannou::noise::Perlin::new().get([x as f64 / 40.0, x as f64 / 83.0]) as f32
            * app.window_rect().h()
            / 2.0;

        model.points.push(Point2 { x, y });
    }

    fn view(app: &App, model: &Self, frame: Frame) -> Frame {
        use nannou::color;

        frame.clear(color::WHITE);

        let draw = app.draw();

        for (src, target) in model.points.iter().zip(model.points.iter().skip(1)) {
            draw.line()
                .thickness(3.0)
                .color(color::BLACK)
                .start(*src)
                .end(*target);
        }

        draw.to_frame(app, &frame).unwrap();

        frame
    }
}

doodle::main!(Model);
