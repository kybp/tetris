extern crate rand;

use std::slice::Iter;

use graphics::{ Context, Graphics };
use graphics::math::Scalar;
use graphics::rectangle::{ Border, Rectangle };
use graphics::rectangle::Shape::Square;
use graphics::types::Color;
use rand::Rng;

use direction::Direction;

const CELL_SIZE:   Scalar = 30.0;
const CELL_BORDER: Scalar = CELL_SIZE / 10.0;

pub fn cells(n: u32) -> Scalar {
    n as Scalar * CELL_SIZE
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub x: Scalar,
    pub y: Scalar,
    size:  Scalar,
    rect:  Rectangle,
}

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

    pub fn draw<G: Graphics>(&self, c: Context, g: &mut G) {
        use graphics::rectangle::square;
        self.rect.draw(square(self.x, self.y, self.size),
                       &c.draw_state, c.transform, g)
    }

    pub fn move_in_direction(&mut self, direction: Direction) {
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
            self.x + cells(1) <= cells(::BOARD_CELL_WIDTH)  + CELL_BORDER &&
            self.y + cells(1) <= cells(::BOARD_CELL_HEIGHT) + CELL_BORDER &&
            ! placed_cells.iter().any(|row| {
                row.iter().any(|cell| {
                    self.x == cell.x && self.y == cell.y
                })
            })
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum BlockShape { I, J, L, O, S, T, Z }

#[derive(Clone)]
pub struct Block {
    pub shape:    BlockShape,
    cells:        [Cell; 4],
    origin_index: Option<usize>,
}

impl Block {
    pub fn draw<G: Graphics>(&self, c: Context, g: &mut G) {
        for cell in self.cells.iter() {
            cell.draw(c, g)
        }
    }

    pub fn iter_cells(&self) -> Iter<Cell> {
        self.cells.iter()
    }

    pub fn try_rotate(&mut self, placed_cells: &Vec<Vec<Cell>>) {
        let mut rotated = self.clone();
        rotated.rotate();
        if rotated.valid(placed_cells) {
            *self = rotated;
        }
    }

    fn rotate(&mut self) {
        if self.shape == BlockShape::O {
            return
        }

        let i = self.origin_index.unwrap();
        let origin_x = self.cells[i].x;
        let origin_y = self.cells[i].y;

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

    pub fn can_move_in_direction(
        &self, direction: Direction, placed_cells: &Vec<Vec<Cell>>
    ) -> bool {
        self.cells.iter().all(|cell| {
            cell.can_move_in_direction(direction, &placed_cells)
        })
    }

    pub fn try_move_in_direction(
        &mut self, direction: Direction, placed_cells: &Vec<Vec<Cell>>
    ) {
        if self.can_move_in_direction(direction, placed_cells) {
            self.move_in_direction(direction);
        }
    }

    pub fn move_in_direction(&mut self, direction: Direction) {
        for cell in self.cells.iter_mut() {
            cell.move_in_direction(direction);
        }
    }

    fn i(x: Scalar, y: Scalar) -> Block {
        let color = [0.4, 0.4, 0.4, 0.7];

        Block {
            shape: BlockShape::I,
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
            shape: BlockShape::L,
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
            shape: BlockShape::O,
            origin_index: None,
            cells: [
                Cell::new(x,            y,            color),
                Cell::new(x + cells(1), y,            color),
                Cell::new(x,            y + cells(1), color),
                Cell::new(x + cells(1), y + cells(1), color),
            ]
        }
    }

    fn j(x: Scalar, y: Scalar) -> Block {
        let color = [0.4, 0.2, 0.0, 0.7];

        Block {
            shape: BlockShape::J,
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
            shape: BlockShape::S,
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
            shape: BlockShape::T,
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
            shape: BlockShape::Z,
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

pub fn random_block(x: Scalar, y: Scalar) -> Block {
    match rand::thread_rng().gen_range(0, 7) {
        0 => Block::i(x, y),
        3 => Block::j(x, y),
        1 => Block::l(x, y),
        2 => Block::o(x, y),
        4 => Block::s(x, y),
        5 => Block::t(x, y),
        6 => Block::z(x, y),
        _ => panic!("Invalid random block"),
    }
}
