use nannou::prelude::*;
struct Model {
    points: Vec<Point2>,
    since_last_tick: std::time::Duration,
}

impl Model {
    fn new(app: &App) -> Self {
        app.new_window().build().unwrap();

        Model {
            points: Vec::with_capacity(20_000),
            since_last_tick: std::time::Duration::from_secs(0),
        }
    }

    fn should_tick(&self) -> bool {
        self.since_last_tick > std::time::Duration::from_millis(20)
    }

    fn tick(&mut self, app: &App) {
        use rand::Rng;

        let wr = app.window_rect();
        let w = wr.w();
        let h = wr.h();
        let mut rng = nannou::rand::prelude::ThreadRng::default();
        eprintln!(
            "sample: {:?}",
            rng.sample(nannou::rand::distributions::Normal::new(
                w as f64 / 2.0,
                w as f64 / 4.0,
            ))
        );
        self.points.push(Point2 {
            x: rng.sample(nannou::rand::distributions::Normal::new(
                0.0,
                w as f64 / 10.0,
            )) as f32,
            y: rng.sample(nannou::rand::distributions::Normal::new(
                0.0,
                h as f64 / 10.0,
            )) as f32,
        })
    }

    fn update(app: &App, model: &mut Self, update: Update) {
        model.since_last_tick += update.since_last;

        if model.should_tick() {
            model.tick(app)
        }
    }

    fn view(app: &App, model: &Self, frame: Frame) -> Frame {
        use nannou::color;

        frame.clear(color::WHITE);

        let draw = app.draw();

        for point in &model.points {
            draw.ellipse()
                .x(point.x)
                .y(point.y)
                .w(4.0)
                .h(4.0)
                .color(color::RED);
        }

        draw.to_frame(app, &frame).unwrap();

        frame
    }
}

doodle::main!(Model);
