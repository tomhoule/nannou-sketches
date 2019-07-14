use nannou::prelude::*;

struct Model {
    cursor: Point2,
    angle: f32,
    poses: Vec<Point2>,
}

impl Model {
    fn new(app: &App) -> Self {
        app.new_window().build().unwrap();

        Model {
            cursor: Point2 { x: 0.0, y: 0.0 },
            angle: 0.0,
            poses: Vec::with_capacity(20_000),
        }
    }

    fn move_cursor(&mut self, elapsed: std::time::Duration, since_last: std::time::Duration) {
        let radius = elapsed.as_millis() as f32 / 100.0;
        self.angle += since_last.as_millis() as f32 / ((radius + 50.0) * 2.0);

        let drawn_radius = radius + (elapsed.as_micros() as f32 / 30000.0).sin() * (radius / 16.0);
        self.poses.push(Point2 {
            x: drawn_radius * self.angle.cos(),
            y: drawn_radius * self.angle.sin(),
        });

        // angle.sin() = y / radius
        // angle.cos() = a / radius

        self.cursor = Point2 {
            x: radius * self.angle.cos(),
            y: radius * self.angle.sin(),
        }
    }

    fn update(app: &App, model: &mut Self, update: nannou::event::Update) {
        model.move_cursor(app.duration.since_start, update.since_last)
    }

    fn view(app: &App, model: &Self, frame: Frame) -> Frame {
        frame.clear(nannou::color::BLUE);

        let draw = app.draw();

        for (origin, target) in (0..model.poses.len()).zip(1..model.poses.len()) {
            draw.line()
                .start(model.poses[origin])
                .end(model.poses[target])
                .thickness(5.0)
                .color(nannou::color::WHITE);
        }

        draw.to_frame(app, &frame).unwrap();

        frame
    }
}

doodle::main!(Model);
