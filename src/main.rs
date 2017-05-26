extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod block;

use block::cells;

use std::collections::HashMap;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::Events;
use piston::input::{ Button, Key, PressEvent, RenderEvent, UpdateEvent };
use piston::window::WindowSettings;

const BOARD_CELL_HEIGHT: u32 = 20;
const BOARD_CELL_WIDTH:  u32 = 10;

const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

fn main() {
    let opengl = OpenGL::V3_2;
    let height = cells(BOARD_CELL_HEIGHT) as u32;
    let width  = cells(BOARD_CELL_WIDTH)  as u32;
    let mut window: Window =
        WindowSettings::new("Tetris", (width, height))
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();
    let mut gl = GlGraphics::new(opengl);

    let mut active_block = block::random_block(cells(1), cells(1));
    let mut score        = Score::new();
    let mut dt           = 0.0;
    let mut paused       = false;
    let mut placed_cells = Vec::<Vec<block::Cell>>::new();

    let mut events = window.events();
    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            draw(&active_block, &placed_cells, &mut gl, render_args)
        }

        if let Some(update_args) = event.update_args() {
            if paused { continue }

            handle_update(&mut dt, &mut score,
                          &mut active_block, &mut placed_cells,
                          update_args)
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            handle_key(key, &mut paused, &mut active_block, &placed_cells)
        }
    }

    println!("You got {} points.", score.points);
}

fn draw(active_block: &block::Block,
        placed_cells: &Vec<Vec<block::Cell>>,
        gl:           &mut GlGraphics,
        render_args:  piston::input::RenderArgs) {
    gl.draw(render_args.viewport(), |c, g| {
        graphics::clear(BACKGROUND_COLOR, g);
        active_block.draw(c, g);
        for row in placed_cells {
            for cell in row {
                cell.draw(c, g);
            }
        }
    })
}

fn handle_key(key:          Key,
              paused:       &mut bool,
              active_block: &mut block::Block,
              placed_cells: &Vec<Vec<block::Cell>>) {
    use Direction::*;

    if key == Key::Space { *paused = ! *paused }

    if *paused { return }

    match key {
        Key::Left  => active_block.try_move_in_direction(Left,  &placed_cells),
        Key::Right => active_block.try_move_in_direction(Right, &placed_cells),
        Key::Down  => active_block.try_move_in_direction(Down,  &placed_cells),
        Key::Up    => active_block.try_rotate(&placed_cells),
        _ => {}
    }
}

fn handle_update(dt:               &mut f64,
                 mut score:        &mut Score,
                 active_block:     &mut block::Block,
                 mut placed_cells: &mut Vec<Vec<block::Cell>>,
                 update_args:     piston::input::UpdateArgs) {
    use Direction::Down;

    *dt += update_args.dt;
    if *dt >= 0.5 {
        *dt = 0.0;
        if active_block.can_move_in_direction(Down, &placed_cells) {
            active_block.move_in_direction(Down);
        } else {
            *score.counts.get_mut(&active_block.shape).unwrap() += 1;
            place_block(active_block, &mut placed_cells);
            *active_block = block::random_block(cells(1), cells(1));
            clear_filled_lines(&mut placed_cells, &mut score);
        }
    }
}

#[derive(Clone, Copy)]
pub enum Direction { Left, Right, Down }

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

fn place_block(block:        &block::Block,
               placed_cells: &mut Vec<Vec<block::Cell>>) {
    for &cell in block.iter_cells() {
        let mut found_matching_row = false;

        for row in placed_cells.iter_mut() {
            if cell.y == row[0].y {
                row.push(cell);
                found_matching_row = true;
                break
            }
        }

        if ! found_matching_row {
            placed_cells.push(vec![cell])
        }
    }
}

fn clear_filled_lines(placed_cells: &mut Vec<Vec<block::Cell>>,
                      score:        &mut Score) {
    placed_cells.sort_by(|a, b| b[0].y.partial_cmp(&a[0].y).unwrap());

    for i in 0..placed_cells.len() {
        if placed_cells[i].len() == BOARD_CELL_WIDTH as usize {
            for j in i..placed_cells.len() {
                for cell in placed_cells[j].iter_mut() {
                    cell.move_in_direction(Direction::Down);
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
