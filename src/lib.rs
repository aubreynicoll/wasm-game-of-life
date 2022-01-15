mod utils;

use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let cells = (0..width * height)
            .map(|_| {
                if rand::random::<f64>() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
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
}
