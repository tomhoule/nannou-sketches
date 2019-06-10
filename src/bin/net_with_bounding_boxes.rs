use nannou::prelude::*;

const DIST: f32 = 35.0;
const BOX_RADIUS: f32 = DIST / 2.0;
const SPEED: f32 = 0.0005;

struct Node {
    box_center: Point2,
    current_pos: Point2,
    current_target: Point2,
}

impl Node {
    fn new(box_center: Point2) -> Node {
        use nannou::rand::random_range as range;
        let current_pos = Point2 {
            x: range(box_center.x - BOX_RADIUS, box_center.x + BOX_RADIUS),
            y: range(box_center.y - BOX_RADIUS, box_center.y + BOX_RADIUS),
        };
        let current_target = Point2 {
            x: range(box_center.x - BOX_RADIUS, box_center.x + BOX_RADIUS),
            y: range(box_center.y - BOX_RADIUS, box_center.y + BOX_RADIUS),
        };
        Node {
            box_center,
            current_pos,
            current_target,
        }
    }

    fn tick(&mut self, elapsed: std::time::Duration) {
        use nannou::rand::random_range as range;

        if self.current_pos.distance(self.current_target) < 2.0 {
            self.current_target = Point2 {
                x: range(
                    self.box_center.x - BOX_RADIUS,
                    self.box_center.x + BOX_RADIUS,
                ),
                y: range(
                    self.box_center.y - BOX_RADIUS,
                    self.box_center.y + BOX_RADIUS,
                ),
            };
        }

        let new_x = self.current_pos.x
            + (self.current_target.x - self.current_pos.x) * (elapsed.as_millis() as f32 * SPEED);
        let new_y = self.current_pos.y
            + ((self.current_target.y - self.current_pos.y) * (elapsed.as_millis() as f32 * SPEED));
        self.current_pos = Point2 { x: new_x, y: new_y };
    }
}

struct Net {
    w: f32,
    h: f32,
    rows: u32,
    per_row: usize,
    nodes: std::cell::RefCell<Vec<Node>>,
}

impl Net {
    fn new(app: &App) -> Self {
        app.new_window().build().unwrap();

        let window_rect = app.window_rect();

        let per_row = (window_rect.w() / DIST + 1.0) as usize;

        let mut nodes = Vec::with_capacity(per_row * 10);
        let mut cursor = window_rect.top_left();

        let mut rows = 0;

        let is_finished = |cursor: Point2| cursor.y < window_rect.bottom();

        while !is_finished(cursor) {
            nodes.push(Node::new(cursor));

            if nodes.len() % per_row == 0 {
                // next row
                rows += 1;
                cursor.y -= DIST;
                cursor.x = if rows % 2 == 0 {
                    window_rect.left()
                } else {
                    window_rect.left() + (DIST / 2.0)
                };
            } else {
                cursor.x += DIST;
            }
        }

        Net {
            nodes: std::cell::RefCell::new(nodes),
            rows,
            per_row,
            w: window_rect.w(),
            h: window_rect.h(),
        }
    }

    fn tick(&self, elapsed: std::time::Duration) {
        for node in self.nodes.borrow_mut().iter_mut() {
            node.tick(elapsed);
        }
    }

    fn view(app: &App, model: &Self, frame: Frame) -> Frame {
        frame.clear(nannou::color::DARK_BLUE);
        let draw = app.draw();

        let elapsed: std::time::Duration = app.duration.since_prev_update;
        model.tick(elapsed);

        let nodes = model.nodes.borrow();

        for (idx, node) in nodes.iter().enumerate() {
            // draw.ellipse()
            //     .color(nannou::color::RED)
            //     .radius(3.0)
            //     .x(node.current_pos.x)
            //     .y(node.current_pos.y)
            //     .finish()
            //     .unwrap();

            // draw.ellipse()
            //     .color(nannou::color::GREEN)
            //     .radius(3.0)
            //     .x(node.current_target.x)
            //     .y(node.current_target.y)
            //     .finish()
            //     .unwrap();

            // draw.ellipse()
            //     .color(nannou::color::CHARCOAL)
            //     .radius(3.0)
            //     .x(node.box_center.x)
            //     .y(node.box_center.y)
            //     .finish()
            //     .unwrap();

            nodes.get(idx + (model.per_row)).map(|bottom| {
                draw.line()
                    .points(node.current_pos.into(), bottom.current_pos.into());
            });

            if (idx + 1) % model.per_row == 0 {
                continue;
            }

            nodes.get(idx + 1).map(|right| {
                draw.line()
                    .points(node.current_pos.into(), right.current_pos.into());
            });

            // draw.ellipse()
            //     .x(node.point.x)
            //     .y(node.point.y)
            //     .radius(2.0)
            //     .finish()
            //     .unwrap();
        }

        draw.to_frame(app, &frame).unwrap();

        frame
    }
}

fn main() {
    nannou::app(Net::new).view(Net::view).run();
}
