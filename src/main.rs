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
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use rand::Rng;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window =
        WindowSettings::new("Tetris", [640, 480])
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();
    let mut gl = GlGraphics::new(opengl);

    let block = random_block(cells(1), cells(1));

    let mut events = window.events();
    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            gl.draw(render_args.viewport(), |c, gl| {
                graphics::clear([0.0, 0.0, 0.0, 0.0], gl);
                block.draw(c, gl);
            })
        }
    }
}

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
}

enum BlockShape { I, L, O, P, S, T, Z }

struct Block {
    cells: [Cell; 4],
    shape: BlockShape,
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
    fn i(x: Scalar, y: Scalar) -> Block {
        let color = [0.4, 0.4, 0.4, 0.7];

        Block {
            shape: BlockShape::I,
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
            shape: BlockShape::L,
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
            shape: BlockShape::O,
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
            shape: BlockShape::P,
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
            shape: BlockShape::S,
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
            shape: BlockShape::T,
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
            shape: BlockShape::Z,
            cells: [
                Cell::new(x + cells(1), y,            color),
                Cell::new(x,            y + cells(1), color),
                Cell::new(x + cells(1), y + cells(1), color),
                Cell::new(x,            y + cells(2), color),
            ]
        }
    }

    fn draw<G: Graphics>(&self, c: Context, g: &mut G) {
        for cell in self.cells.iter() {
            cell.draw(c, g)
        }
    }
}
