extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod block;
mod direction;

use block::cells;

use std::collections::HashMap;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::Events;
use piston::input::{ Button, Key, PressEvent, RenderEvent, UpdateEvent };
use piston::window::WindowSettings;

const BOARD_CELL_HEIGHT: u32 = 20;
const BOARD_CELL_WIDTH:  u32 = 10;

fn main() {
    use direction::Direction::*;

    let opengl = OpenGL::V3_2;
    let height = cells(BOARD_CELL_HEIGHT) as u32;
    let width  = cells(BOARD_CELL_WIDTH)  as u32;
    let mut window: Window =
        WindowSettings::new("Tetris", (width, height))
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();
    let mut gl = GlGraphics::new(opengl);

    let mut block        = block::random_block(cells(1), cells(1));
    let mut score        = Score::new();
    let mut dt           = 0.0;
    let mut paused       = false;
    let mut placed_cells = Vec::<Vec<block::Cell>>::new();

    let mut events = window.events();
    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            gl.draw(render_args.viewport(), |c, gl| {
                graphics::clear([0.0, 0.0, 0.0, 0.0], gl);
                block.draw(c, gl);
                for row in &placed_cells {
                    for cell in row {
                        cell.draw(c, gl);
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
                    *score.counts.get_mut(&block.shape).unwrap() += 1;
                    for &cell in block.iter_cells() {
                        add_cell(cell, &mut placed_cells);
                    }
                    clear_filled_lines(&mut placed_cells, &mut score);
                    block = block::random_block(cells(1), cells(1));
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

    println!("You got {} points.", score.points);
}

struct Score {
    points: usize,
    lines:  usize,
    counts: HashMap<block::BlockShape, usize>,
}

impl Score {
    fn new() -> Score {
        use block::BlockShape::*;
        let mut counts = HashMap::new();
        counts.insert(I, 0);
        counts.insert(J, 0);
        counts.insert(L, 0);
        counts.insert(O, 0);
        counts.insert(S, 0);
        counts.insert(T, 0);
        counts.insert(Z, 0);
        Score {
            points: 0,
            lines:  0,
            counts: counts,
        }
    }
}

fn add_cell(cell: block::Cell, placed_cells: &mut Vec<Vec<block::Cell>>) {
    for row in placed_cells.iter_mut() {
        if cell.y == row[0].y {
            row.push(cell);
            return;
        }
    }

    placed_cells.push(vec![cell]);
    placed_cells.sort_by(|a, b| b[0].y.partial_cmp(&a[0].y).unwrap());
}

fn clear_filled_lines(
    placed_cells: &mut Vec<Vec<block::Cell>>, score: &mut Score
) {
    for i in 0..placed_cells.len() {
        if placed_cells[i].len() == BOARD_CELL_WIDTH as usize {
            for j in i..placed_cells.len() {
                for cell in placed_cells[j].iter_mut() {
                    cell.move_in_direction(direction::Direction::Down);
                }
            }
        }
    }

    let starting_rows = placed_cells.len();
    placed_cells.retain(|row| row.len() != BOARD_CELL_WIDTH as usize);
    let lines_cleared = starting_rows - placed_cells.len();

    score.points += match lines_cleared {
        1 =>  100,
        2 =>  200,
        3 =>  500,
        4 => 1000,
        _ =>    0,
    };
    score.lines += lines_cleared;
}
