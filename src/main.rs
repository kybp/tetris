extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use graphics::{ Context, Graphics };
use graphics::math::Scalar;
use graphics::rectangle::{ Border, Rectangle };
use graphics::rectangle::Shape::Square;
use graphics::types::Color;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::Events;
use piston::input::{ Button, Key, PressEvent, RenderEvent, UpdateEvent };
use piston::window::WindowSettings;
use rand::Rng;

const BOARD_CELL_HEIGHT: u32 = 20;
const BOARD_CELL_WIDTH:  u32 = 10;

fn main() {
    use Direction::*;

    let opengl = OpenGL::V3_2;
    let height = cells(BOARD_CELL_HEIGHT) as u32;
    let width  = cells(BOARD_CELL_WIDTH)  as u32;
    let mut window: Window =
        WindowSettings::new("Tetris", (width, height))
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();
    let mut gl = GlGraphics::new(opengl);

    let mut block = random_block(cells(1), cells(1));
    let mut dt = 0.0;
    let mut paused = false;
    let mut placed_cells = Vec::<Vec<Cell>>::new();

    let mut events = window.events();
    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            gl.draw(render_args.viewport(), |c, gl| {
                graphics::clear([0.0, 0.0, 0.0, 0.0], gl);
                block.draw(c, gl);
                for row in &placed_cells {
                    for block in row {
                        block.draw(c, gl);
                    }
                }
            })
        }

        if let Some(update_args) = event.update_args() {
            if paused {
                continue;
            }

            dt += update_args.dt;
            if dt >= 0.5 {
                dt = 0.0;
                if block.can_move_in_direction(Down, &placed_cells) {
                    block.move_in_direction(Down);
                } else {
                    for &cell in block.cells.iter() {
                        add_cell(cell, &mut placed_cells);
                    }
                    clear_filled_lines(&mut placed_cells);
                    block = random_block(cells(1), cells(1));
                }
            }
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            if key == Key::Space {
                paused = ! paused;
            }

            if paused {
                continue;
            }

            match key {
                Key::Left
                    if block.can_move_in_direction(Left, &placed_cells) => {
                        block.move_in_direction(Left);
                    },
                Key::Right
                    if block.can_move_in_direction(Right, &placed_cells) => {
                        block.move_in_direction(Right);
                    },
                Key::Down
                    if block.can_move_in_direction(Down, &placed_cells) => {
                        block.move_in_direction(Down);
                    },
                Key::Up => block.try_rotate(&placed_cells),
                _ => {}
            }
        }
    }
}

fn add_cell(cell: Cell, placed_cells: &mut Vec<Vec<Cell>>) {
    for row in placed_cells.iter_mut() {
        if cell.y == row[0].y {
            row.push(cell);
            return;
        }
    }

    placed_cells.push(vec![cell]);
    placed_cells.sort_by(|a, b| b[0].y.partial_cmp(&a[0].y).unwrap());
}

fn clear_filled_lines(placed_cells: &mut Vec<Vec<Cell>>) {
    for i in 0..placed_cells.len() {
        if placed_cells[i].len() == BOARD_CELL_WIDTH as usize {
            for j in i..placed_cells.len() {
                for cell in placed_cells[j].iter_mut() {
                    cell.move_in_direction(Direction::Down);
                }
            }
        }
    }

    placed_cells.retain(|row| row.len() != BOARD_CELL_WIDTH as usize);
}

#[derive(Clone, Copy)]
struct Cell {
    x:    Scalar,
    y:    Scalar,
    size: Scalar,
    rect: Rectangle,
}

const CELL_SIZE:   Scalar = 30.0;
const CELL_BORDER: Scalar = CELL_SIZE / 10.0;

fn cells(n: u32) -> Scalar {
    n as Scalar * CELL_SIZE
}

#[derive(Clone, Copy)]
enum Direction { Left, Right, Down }

impl Cell {
    fn new(x: Scalar, y: Scalar, color: Color) -> Cell {
        let mut border_color = color;
        border_color[3] -= 0.3;

        Cell {
            x: x + CELL_BORDER, y: y + CELL_BORDER,
            size: CELL_SIZE - CELL_BORDER * 2.0,
            rect: Rectangle {
                color: color,
                shape: Square,
                border: Some(Border {
                    color:  border_color,
                    radius: CELL_BORDER,
                })
            }
        }
    }

    fn draw<G: Graphics>(&self, c: Context, g: &mut G) {
        use graphics::rectangle::square;
        self.rect.draw(square(self.x, self.y, self.size),
                       &c.draw_state, c.transform, g)
    }

    fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Down  => self.y += cells(1),
            Direction::Left  => self.x -= cells(1),
            Direction::Right => self.x += cells(1),
        }
    }

    fn can_move_in_direction(
        &self, direction: Direction, placed_cells: &Vec<Vec<Cell>>
    ) -> bool {
        let mut moved = self.clone();
        moved.move_in_direction(direction);
        moved.valid(placed_cells)
    }

    fn valid(&self, placed_cells: &Vec<Vec<Cell>>) -> bool {
        self.x >= 0.0 &&
            self.x + cells(1) <= cells(BOARD_CELL_WIDTH)  + CELL_BORDER &&
            self.y + cells(1) <= cells(BOARD_CELL_HEIGHT) + CELL_BORDER &&
            ! placed_cells.iter().any(|row| {
                row.iter().any(|cell| {
                    self.x == cell.x && self.y == cell.y
                })
            })
    }
}

#[derive(Clone)]
struct Block {
    origin_index: Option<usize>,
    cells: [Cell; 4],
}

fn random_block(x: Scalar, y: Scalar) -> Block {
    match rand::thread_rng().gen_range(0, 7) {
        0 => Block::i(x, y),
        1 => Block::l(x, y),
        2 => Block::o(x, y),
        3 => Block::p(x, y),
        4 => Block::s(x, y),
        5 => Block::t(x, y),
        6 => Block::z(x, y),
        _ => panic!("Invalid random block"),
    }
}

impl Block {
    fn draw<G: Graphics>(&self, c: Context, g: &mut G) {
        for cell in self.cells.iter() {
            cell.draw(c, g)
        }
    }

    fn try_rotate(&mut self, placed_cells: &Vec<Vec<Cell>>) {
        let mut rotated = self.clone();
        rotated.rotate();
        if rotated.valid(placed_cells) {
            *self = rotated;
        }
    }

    fn rotate(&mut self) {
        let origin_x;
        let origin_y;

        if let Some(i) = self.origin_index {
            origin_x = self.cells[i].x;
            origin_y = self.cells[i].y;
        } else {
            return;
        }

        for (i, cell) in self.cells.iter_mut().enumerate() {
            if Some(i) == self.origin_index {
                continue
            }

            if cell.x > origin_x && cell.y > origin_y {
                cell.x -= cells(2);
                continue;
            } else if cell.x > origin_x && cell.y < origin_y {
                cell.y += cells(2);
                continue;
            } else if cell.x < origin_x && cell.y < origin_y {
                cell.x += cells(2);
                continue;
            } else if cell.x < origin_x && cell.y > origin_y {
                cell.y -= cells(2);
                continue;
            }

            // For the cell not adjacent to the origin in an I-block,
            // we want to scale the movement by 2.
            let scale = (if cell.x == origin_x {
                cell.y - origin_y
            } else {
                cell.x - origin_x
            } / CELL_SIZE).abs() as u32;

            if cell.x < origin_x {
                cell.x += cells(scale);
                cell.y -= cells(scale);
            } else if cell.x > origin_x {
                cell.x -= cells(scale);
                cell.y += cells(scale);
            } else if cell.y < origin_y {
                cell.x += cells(scale);
                cell.y += cells(scale);
            } else if cell.y > origin_y {
                cell.x -= cells(scale);
                cell.y -= cells(scale);
            }
        }
    }

    fn valid(&self, placed_cells: &Vec<Vec<Cell>>) -> bool {
        self.cells.iter().all(|cell| cell.valid(placed_cells))
    }

    fn can_move_in_direction(
        &self, direction: Direction, placed_cells: &Vec<Vec<Cell>>
    ) -> bool {
        self.cells.iter().all(|cell| {
            cell.can_move_in_direction(direction, &placed_cells)
        })
    }

    fn move_in_direction(&mut self, direction: Direction) {
        for cell in self.cells.iter_mut() {
            cell.move_in_direction(direction);
        }
    }

    fn i(x: Scalar, y: Scalar) -> Block {
        let color = [0.4, 0.4, 0.4, 0.7];

        Block {
            origin_index: Some(1),
            cells: [
                Cell::new(x, y + cells(0), color),
                Cell::new(x, y + cells(1), color),
                Cell::new(x, y + cells(2), color),
                Cell::new(x, y + cells(3), color),
            ]
        }
    }

    fn l(x: Scalar, y: Scalar) -> Block {
        let color = [0.6, 0.6, 0.1, 0.7];

        Block {
            origin_index: Some(2),
            cells: [
                Cell::new(x,            y,            color),
                Cell::new(x + cells(1), y,            color),
                Cell::new(x + cells(1), y + cells(1), color),
                Cell::new(x + cells(1), y + cells(2), color),
            ]
        }
    }

    fn o(x: Scalar, y: Scalar) -> Block {
        let color = [0.7, 0.0, 0.7, 0.7];

        Block {
            origin_index: None,
            cells: [
                Cell::new(x,            y,            color),
                Cell::new(x + cells(1), y,            color),
                Cell::new(x,            y + cells(1), color),
                Cell::new(x + cells(1), y + cells(1), color),
            ]
        }
    }

    fn p(x: Scalar, y: Scalar) -> Block {
        let color = [0.4, 0.2, 0.0, 0.7];

        Block {
            origin_index: Some(2),
            cells: [
                Cell::new(x,            y,            color),
                Cell::new(x + cells(1), y,            color),
                Cell::new(x,            y + cells(1), color),
                Cell::new(x,            y + cells(2), color),
            ]
        }
    }

    fn s(x: Scalar, y: Scalar) -> Block {
        let color = [0.0, 0.0, 0.8, 0.7];

        Block {
            origin_index: Some(1),
            cells: [
                Cell::new(x,            y,            color),
                Cell::new(x,            y + cells(1), color),
                Cell::new(x + cells(1), y + cells(1), color),
                Cell::new(x + cells(1), y + cells(2), color),
            ]
        }
    }

    fn t(x: Scalar, y: Scalar) -> Block {
        let color = [0.6, 0.0, 0.0, 0.7];

        Block {
            origin_index: Some(1),
            cells: [
                Cell::new(x,            y,            color),
                Cell::new(x,            y + cells(1), color),
                Cell::new(x + cells(1), y + cells(1), color),
                Cell::new(x,            y + cells(2), color),
            ]
        }
    }

    fn z(x: Scalar, y: Scalar) -> Block {
        let color = [0.0, 0.7, 0.3, 0.7];

        Block {
            origin_index: Some(1),
            cells: [
                Cell::new(x + cells(1), y,            color),
                Cell::new(x,            y + cells(1), color),
                Cell::new(x + cells(1), y + cells(1), color),
                Cell::new(x,            y + cells(2), color),
            ]
        }
    }
}
