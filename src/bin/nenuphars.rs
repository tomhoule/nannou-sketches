///! inspired by https://www.instagram.com/p/Ba9ApsdFlnV/
use nannou::draw::properties::color::IntoRgba;
use nannou::prelude::*;

const POPUP_INTERVAL: std::time::Duration = std::time::Duration::from_millis(500);

/// The lifetime one px of radius is equivalent to.
const RADIUS_PX_TO_LIFETIME: std::time::Duration = std::time::Duration::from_millis(300);

#[derive(Debug, PartialEq)]
enum NenupharState {
    Live,
    Dead,
    Dying,
}

#[derive(PartialEq)]
enum Palette {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Palette {
    fn random_except(&self) -> Palette {
        loop {
            let new = Palette::random();
            if &new != self {
                return new;
            }
        }
    }

    fn random() -> Self {
        match nannou::rand::random_range(0, 5) {
            0 => Palette::One,
            1 => Palette::Two,
            2 => Palette::Three,
            3 => Palette::Four,
            4 => Palette::Five,
            _ => panic!("color out of range"),
        }
    }

    fn to_rgba(&self) -> nannou::color::Rgba {
        match self {
            Palette::One => nannou::color::Rgb::new_u8(74, 78, 77),
            Palette::Two => nannou::color::Rgb::new_u8(14, 154, 167),
            Palette::Three => nannou::color::Rgb::new_u8(61, 164, 171),
            Palette::Four => nannou::color::Rgb::new_u8(246, 205, 97),
            Palette::Five => nannou::color::Rgb::new_u8(254, 138, 113),
        }
        .into_rgba()
    }
}

struct Nenuphar {
    color: Palette,
    center: Point2,
    apparent_radius: u32,
    radius: u32,
    state: NenupharState,
    lifetime: std::time::Duration,
    nested: Vec<Nenuphar>,
}

impl Nenuphar {
    fn new(center: Point2, radius: u32, parent_color: &Palette) -> Nenuphar {
        Nenuphar {
            color: parent_color.random_except(),
            apparent_radius: 0,
            radius,
            center,
            state: NenupharState::Live,
            lifetime: std::time::Duration::from_millis(0),
            nested: Vec::with_capacity(5),
        }
    }

    fn overlaps(&self, other: &Nenuphar) -> bool {
        let a_squared = (other.center.y - self.center.y).abs().powi(2);
        let b_squared = (other.center.x - self.center.x).abs().powi(2);
        let dist = (a_squared + b_squared).sqrt();
        dist < (self.radius + other.radius) as f32
    }

    fn pop_inner(&mut self) {
        if self.radius < 20 {
            return;
        }

        let radius = nannou::rand::random_range(10, (self.radius as f32 * 0.7) as u32);
        let dist = nannou::rand::random_range(1.0, (self.radius - radius) as f32);
        let angle = nannou::rand::random_range(0.0, std::f32::consts::PI * 2.0);

        let center = Point2 {
            x: self.center.x + dist * angle.cos(),
            y: self.center.y + dist * angle.sin(),
        };

        let nenuphar = Nenuphar::new(center, radius, &self.color);

        if !self.nested.iter().any(|n| n.overlaps(&nenuphar)) {
            self.nested.push(nenuphar);
        }
    }

    fn update(&mut self, elapsed: std::time::Duration) {
        self.lifetime += elapsed;

        match self.state {
            NenupharState::Live => {
                if self.apparent_radius <= self.radius {
                    self.apparent_radius = std::cmp::min(
                        self.radius,
                        self.apparent_radius + (elapsed.as_millis() as u32 / 6),
                    );
                }

                for nested in &mut self.nested {
                    nested.update(elapsed)
                }

                self.pop_inner();

                if self.lifetime.as_millis()
                    > (RADIUS_PX_TO_LIFETIME.as_millis() * self.radius as u128)
                {
                    self.state = NenupharState::Dying;
                }
            }
            NenupharState::Dead => (),
            NenupharState::Dying => {
                if self.apparent_radius > 0 {
                    self.apparent_radius -= 1;
                } else {
                    self.state = NenupharState::Dead;
                }
            }
        }
    }

    fn draw(&self, draw: &nannou::app::Draw) {
        draw.ellipse()
            .x_y(self.center.x, self.center.y)
            .color(self.color.to_rgba())
            .w(self.apparent_radius as f32 * 2.0)
            .h(self.apparent_radius as f32 * 2.0)
            .finish()
            .expect("drawing a nenuphar");

        for nested in &self.nested {
            nested.draw(draw);
        }
    }
}

struct Model {
    nenuphars: Vec<Nenuphar>,
    elapsed: std::time::Duration,
}

impl Model {
    fn new(app: &App) -> Self {
        app.new_window().build().unwrap();
        Model {
            nenuphars: Vec::with_capacity(10),
            elapsed: std::time::Duration::from_millis(0),
        }
    }

    fn view(app: &App, model: &Model, frame: Frame) -> Frame {
        frame.clear(Self::bg_color().to_rgba());

        let draw = app.draw();

        for nenuphar in model
            .nenuphars
            .iter()
            .filter(|n| n.state != NenupharState::Dead)
        {
            nenuphar.draw(&draw);
        }

        draw.to_frame(app, &frame).unwrap();

        frame
    }

    fn update(app: &App, model: &mut Model, update: nannou::event::Update) {
        model.elapsed += update.since_last;

        for nenuphar in &mut model.nenuphars {
            nenuphar.update(update.since_last);
        }

        if model.elapsed >= POPUP_INTERVAL {
            model.elapsed = std::time::Duration::from_millis(0);
            model.pop_nenuphar(app.window_rect())
        }
    }

    fn bg_color() -> Palette {
        Palette::Four
    }

    fn pop_nenuphar(&mut self, window_rect: nannou::geom::rect::Rect) {
        let radius = nannou::rand::random_range(20, (window_rect.h() / 3.0).floor() as u32);
        let position = Point2 {
            x: nannou::rand::random_range(window_rect.left(), window_rect.right()),
            y: nannou::rand::random_range(window_rect.bottom(), window_rect.top()),
        };

        let nenuphar = Nenuphar::new(position, radius, &Self::bg_color());

        if !self
            .nenuphars
            .iter()
            .filter(|n| n.state != NenupharState::Dead)
            .any(|n| n.overlaps(&nenuphar))
        {
            self.nenuphars.push(nenuphar);
        }
    }
}

fn main() -> Result<(), failure::Error> {
    nannou::app(Model::new)
        .view(Model::view)
        .update(Model::update)
        .run();

    Ok(())
}
