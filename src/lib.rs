mod utils;

use js_sys::Math;
use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(random: bool) -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;
        let cells = match random {
            true => (0..width * height)
                .map(|_| {
                    if Math::random() < 0.5 {
                        Cell::Dead
                    } else {
                        Cell::Alive
                    }
                })
                .collect(),
            false => (0..width * height).map(|_| Cell::Dead).collect(),
        };

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
        self.cells = (0..self.width * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
        self.cells = (0..self.height * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        let i = self.get_index(row, col);
        self.cells[i].toggle()
    }

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let alive_neighbors = self.count_alive_neighbors(row, col);
                let next_cell = match (cell, alive_neighbors) {
                    (Cell::Alive, 0 | 1) => Cell::Dead,
                    (Cell::Alive, 2 | 3) => Cell::Alive,
                    (Cell::Alive, _) => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };
                next[index] = next_cell;
            }
        }
        self.cells = next;
    }
}

impl Universe {
    fn count_alive_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for row_offset in [self.height - 1, 0, 1] {
            for col_offset in [self.width - 1, 0, 1] {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }
                let row = (row + row_offset) % self.height;
                let col = (col + col_offset) % self.width;
                let i = self.get_index(row, col);
                count += self.cells[i] as u8;
            }
        }
        count
    }

    pub fn borrow_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for &(row, col) in cells {
            let i = self.get_index(row, col);
            self.cells[i] = Cell::Alive;
        }
    }
}
