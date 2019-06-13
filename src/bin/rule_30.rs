use nannou::prelude::*;

const GAP: f32 = 20.0;

type Error = Box<dyn std::error::Error>;

struct Point {
    coord: Point2,
    state: bool,
    color: u8,
}

impl Point {
    fn current_color(&self) -> nannou::color::Rgba {
        let color = self.color as f32;
        nannou::color::Rgba::new(color, color, color, 0.5)
    }
}

struct Lattice {
    _w: f32,
    _h: f32,
    points: Vec<Point>,
    elapsed: std::time::Duration,
}

impl Lattice {
    fn new(app: &App) -> Result<Self, Error> {
        app.new_window().build()?;

        let window_rect = app.window_rect();

        let per_row = window_rect.w() / GAP;
        let mut points = Vec::new();

        let mut cursor: Point2 = window_rect.top_left();

        let locations = std::iter::once(cursor).chain(std::iter::from_fn(|| {
            if cursor.x > window_rect.right() {
                cursor.x = window_rect.left();
                cursor.y -= GAP;
                return Some(cursor);
            }

            if cursor.y < window_rect.bottom() {
                return None;
            }

            cursor.x += GAP;
            Some(cursor)
        }));

        for location in locations {
            points.push(Point {
                coord: location,
                state: false,
                color: 0,
            });
        }

        for _ in 1..(points.len() / 5) {
            let idx = nannou::rand::random_range(0, points.len() - 1);
            points[idx].state = true;
        }

        Ok(Lattice {
            points,
            _w: window_rect.w(),
            _h: window_rect.h(),
            elapsed: std::time::Duration::new(0, 0),
        })
    }

    fn view(app: &App, model: &Self, frame: Frame) -> Frame {
        let draw = app.draw();

        frame.clear(nannou::color::BLUE);

        for point in model.points.iter() {
            eprintln!("{:?}", point.current_color());
            draw.ellipse()
                .color(point.current_color())
                .h(GAP * 1.4)
                .w(GAP * 1.4)
                .x(point.coord.x)
                .y(point.coord.y)
                .finish()
                .unwrap();
        }

        draw.to_frame(app, &frame).unwrap();

        frame
    }

    fn update(app: &App, model: &mut Self, update: nannou::event::Update) {
        if model.elapsed.as_millis() > 1000 / 10 {
            model.elapsed = update.since_last;

            for _ in 0..20 {
                let idx = nannou::rand::random_range(0, model.points.len() - 1);
                model.points[idx].state = !model.points[idx].state;
            }

            model.tick();
        } else {
            model.elapsed += update.since_last;
        }

        for point in model.points.iter_mut() {
            let increment = (update.since_last.as_millis() % 2) as u8;
            if point.state {
                point.color = point.color.saturating_add(increment);
            } else {
                point.color = point.color.saturating_sub(increment);
            }
        }
    }

    fn state_at(&self, idx: usize) -> bool {
        self.points.get(idx).map(|val| val.state).unwrap_or(false)
    }

    fn tick(&mut self) {
        for idx in 0..self.points.len() {
            let new_val = match (
                self.state_at(if idx == 0 { 0 } else { idx - 1 }),
                self.state_at(idx),
                self.state_at(idx + 1),
            ) {
                (true, true, true) => false,
                (true, true, false) => false,
                (true, false, true) => false,
                (true, false, false) => true,
                (false, true, true) => true,
                (false, true, false) => true,
                (false, false, true) => true,
                (false, false, false) => false,
            };
            self.points[idx].state = new_val;
        }
    }
}

fn main() -> Result<(), Error> {
    nannou::app(|app| Lattice::new(app).unwrap())
        .view(Lattice::view)
        .update(Lattice::update)
        .run();

    Ok(())
}
