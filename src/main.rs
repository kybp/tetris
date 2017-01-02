extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

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

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window =
        WindowSettings::new("Tetris", [640, 480])
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();
    let mut gl = GlGraphics::new(opengl);

    let block = Block::new(50.0, 50.0, [0.4, 0.4, 0.0, 0.7]);

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

const CELL_SIZE: Scalar = 50.0;

impl Cell {
    fn new(x: Scalar, y: Scalar, color: Color) -> Cell {
        let border_radius = 5.0;
        let mut border_color = color;
        border_color[3] -= 0.3;

        Cell {
            x: x, y: y, size: CELL_SIZE - border_radius * 2.0,
            rect: Rectangle {
                color: color,
                shape: Square,
                border: Some(Border {
                    color: border_color,
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

struct Block {
    cells: [Cell; 4],
}

impl Block {
    fn new(x: Scalar, y: Scalar, color: Color) -> Block {
        let cell_size = 50.0;
        Block {
            cells: [
                Cell::new(x, y + CELL_SIZE * 0.0, color),
                Cell::new(x, y + CELL_SIZE * 1.0, color),
                Cell::new(x, y + CELL_SIZE * 2.0, color),
                Cell::new(x, y + CELL_SIZE * 3.0, color),
            ]
        }
    }

    fn draw<G: Graphics>(&self, c: Context, g: &mut G) {
        for cell in self.cells.iter() {
            cell.draw(c, g)
        }
    }
}
