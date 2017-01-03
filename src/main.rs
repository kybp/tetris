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

    let mut placed_blocks = Vec::<Block>::new();

    let mut events = window.events();
    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            gl.draw(render_args.viewport(), |c, gl| {
                graphics::clear([0.0, 0.0, 0.0, 0.0], gl);
                block.draw(c, gl);
                for block in &placed_blocks {
                    block.draw(c, gl);
                }
            })
        }

        if let Some(update_args) = event.update_args() {
            dt += update_args.dt;
            if dt >= 0.5 {
                dt = 0.0;
                if block.can_move_in_direction(Down, &placed_blocks) {
                    block.move_in_direction(Down);
                } else {
                    placed_blocks.push(block);
                    block = random_block(cells(1), cells(1));
                }
            }
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Left
                    if block.can_move_in_direction(Left, &placed_blocks) => {
                        block.move_in_direction(Left);
                    },
                Key::Right
                    if block.can_move_in_direction(Right, &placed_blocks) => {
                        block.move_in_direction(Right);
                    },
                _ => {}
            }
        }
    }
}

#[derive(Clone)]
struct Cell {
    x:    Scalar,
    y:    Scalar,
    size: Scalar,
    rect: Rectangle,
}

const CELL_SIZE: Scalar = 30.0;

fn cells(n: u32) -> Scalar {
    n as Scalar * CELL_SIZE
}

#[derive(Clone, Copy)]
enum Direction { Left, Right, Down }

impl Cell {
    fn new(x: Scalar, y: Scalar, color: Color) -> Cell {
        let border_radius = CELL_SIZE / 10.0;
        let mut border_color = color;
        border_color[3] -= 0.3;

        Cell {
            x: x, y: y, size: CELL_SIZE - border_radius * 2.0,
            rect: Rectangle {
                color: color,
                shape: Square,
                border: Some(Border {
                    color:  border_color,
                    radius: border_radius,
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
        &self, direction: Direction, placed_blocks: &Vec<Block>
    ) -> bool {
        let mut moved = self.clone();
        moved.move_in_direction(direction);
        moved.x >= 0.0 &&
            moved.x + cells(1) <= cells(BOARD_CELL_WIDTH) &&
            moved.y + cells(1) <= cells(BOARD_CELL_HEIGHT) &&
            ! placed_blocks.iter().any(|block| block.contains(&moved))
    }
}

struct Block {
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

    fn contains(&self, cell: &Cell) -> bool {
        self.cells.iter().any(|other| {
            cell.x == other.x && cell.y == other.y
        })
    }

    fn can_move_in_direction(
        &self, direction: Direction, placed_blocks: &Vec<Block>
    ) -> bool {
        self.cells.iter().all(|cell| {
            cell.can_move_in_direction(direction, &placed_blocks)
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
            cells: [
                Cell::new(x,            y,            color),
                Cell::new(x + cells(1), y,            color),
                Cell::new(x,            y + cells(1), color),
                Cell::new(x + cells(1), y + cells(1), color),
            ]
        }
    }

    fn p(x: Scalar, y: Scalar) -> Block {
        let color = [0.4, 0.3, 0.0, 0.7];

        Block {
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
            cells: [
                Cell::new(x + cells(1), y,            color),
                Cell::new(x,            y + cells(1), color),
                Cell::new(x + cells(1), y + cells(1), color),
                Cell::new(x,            y + cells(2), color),
            ]
        }
    }
}
