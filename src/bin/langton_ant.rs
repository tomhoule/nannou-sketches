use nannou::prelude::*;

/// The length of the side of one square in the grid, in pixels.
const SIZE: usize = 9;
const SIZE_FLOAT: f32 = SIZE as f32;

const TICK_INTERVAL: std::time::Duration = std::time::Duration::from_millis(12);

#[derive(Debug)]
enum CellState {
    Black,
    White,
}

#[derive(Debug)]
struct Cell {
    state: CellState,
    pos: Point2,
}

impl Cell {
    fn new<N: Into<f32>>(x: N, y: N) -> Self {
        Cell {
            state: CellState::White,
            pos: Point2 {
                x: x.into(),
                y: y.into(),
            },
        }
    }

    fn flip(&mut self) {
        self.state = match self.state {
            CellState::Black => CellState::White,
            CellState::White => CellState::Black,
        };
    }
}

#[derive(Debug)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    fn random() -> Self {
        match nannou::rand::random_range(0, 4) {
            0 => Direction::Top,
            1 => Direction::Right,
            2 => Direction::Bottom,
            3 => Direction::Left,
            _ => panic!("direction out of range"),
        }
    }
}

#[derive(Debug)]
struct Ant {
    direction: Direction,
    /// Index of the Cell.
    position: usize,
}

impl Ant {
    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Top => Direction::Left,
            Direction::Right => Direction::Top,
            Direction::Bottom => Direction::Right,
            Direction::Left => Direction::Bottom,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Top,
        }
    }
}

#[derive(Debug)]
struct Model {
    elapsed: std::time::Duration,
    cells: Vec<Cell>,
    ants: Vec<Ant>,
    per_row: usize,
}

impl Model {
    fn new(app: &App) -> Self {
        app.new_window().build().unwrap();
        let window_rect = app.window_rect();

        let mut cells = Vec::with_capacity(200);

        for y in
            ((window_rect.bottom().floor() as i16)..window_rect.top().floor() as i16).step_by(SIZE)
        {
            for x in ((window_rect.left() as i16)..window_rect.right().floor() as i16).step_by(SIZE)
            {
                cells.push(Cell::new(x, y))
            }
        }

        let num_ants = 6;
        let mut ants = Vec::with_capacity(num_ants);

        for _ in 0..num_ants {
            let random_cell_idx = nannou::rand::random_range(0, cells.len());

            ants.push(Ant {
                direction: Direction::random(),
                position: random_cell_idx,
            })
        }

        // cells.get_mut(random_cell_idx).map(|cell| {
        //     cell.flip();
        // });
        //

        Model {
            cells,
            elapsed: Default::default(),
            ants,
            per_row: window_rect.w() as usize / SIZE,
        }
    }

    fn tick(&mut self) {
        for ant in &mut self.ants {
            // - At a white square, turn 90° right, flip the color of the square, move
            // forward one unit
            // - At a black square, turn 90° left, flip the color of the square, move
            // forward one unit
            let cell = self
                .cells
                .get_mut(ant.position)
                .expect("Cell is out of bounds.");

            cell.flip();

            match cell.state {
                CellState::Black => {
                    ant.turn_left();
                }
                CellState::White => {
                    ant.turn_right();
                }
            }

            let col_idx = ant.position % self.per_row;

            let new_idx = match ant.direction {
                Direction::Top => {
                    if ant.position < self.per_row {
                        col_idx
                    } else {
                        ant.position - self.per_row
                    }
                }
                Direction::Left => {
                    if col_idx == 0 {
                        ant.position + (self.per_row - 1)
                    } else {
                        ant.position - 1
                    }
                }
                Direction::Right => {
                    if col_idx == self.per_row - 1 {
                        ant.position - (self.per_row - 1)
                    } else {
                        ant.position + 1
                    }
                }
                Direction::Bottom => (ant.position + self.per_row) % self.cells.len(),
            };

            // println!(
            //     "old idx: {:?}, dir: {:?}, new_idx: {:?}",
            //     ant.position, ant.direction, new_idx
            // );
            ant.position = new_idx;
        }
    }

    fn update(_app: &App, model: &mut Self, update: nannou::event::Update) {
        if model.elapsed > TICK_INTERVAL {
            model.elapsed = update.since_last;
            model.tick()
        } else {
            model.elapsed += update.since_last;
        }
    }

    fn view(app: &App, model: &Self, frame: Frame) -> Frame {
        frame.clear(nannou::color::BLUE);
        let draw = app.draw();

        for cell in model.cells.iter() {
            // eprintln!("drawing cell {:?}", cell);
            draw.rect()
                .x(cell.pos.x - SIZE_FLOAT / 2.0)
                .y(cell.pos.y + SIZE_FLOAT / 2.0)
                .w(SIZE_FLOAT / 1.5)
                .h(SIZE_FLOAT / 1.5)
                .color(match cell.state {
                    CellState::Black => nannou::color::RED,
                    CellState::White => nannou::color::WHITE,
                })
                .finish()
                .unwrap();
        }

        draw.to_frame(app, &frame).unwrap();

        frame
    }
}

fn main() -> Result<(), failure::Error> {
    nannou::app(Model::new)
        .view(Model::view)
        .update(Model::update)
        .run();

    Ok(())
}
