mod utils;

use wasm_bindgen::prelude::*;

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
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    underpopulation: u8,
    overpopulation: u8,
    reproduction: u8,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let cells: Vec<Cell> = (0..width * height)
            .map(|_i| Cell::Dead).collect();
        let underpopulation = 2;
        let overpopulation = 3;
        let reproduction = 3;

            return Universe {
                width,
                height,
                cells,
                underpopulation,
                overpopulation,
                reproduction,
            };
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                let next_cell = match(cell, live_neighbors) {
                    (Cell::Alive, x)  if x >= self.underpopulation && x <= self.overpopulation => Cell::Alive,
                    (Cell::Alive, x) if x < self.underpopulation => Cell::Dead,
                    (Cell::Alive, x) if x > self.overpopulation => Cell::Dead,
                    (Cell::Dead, x) if x == self.reproduction => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }

    pub fn cells(&self) -> *const Cell {
        return self.cells.as_ptr();
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn set_overpopulation(&mut self, overpopulation: u8) {
        self.overpopulation = overpopulation;
    }

    pub fn set_underpopulation(&mut self, underpopulation: u8) {
        self.underpopulation = underpopulation;
    }

    pub fn set_reproduction(&mut self, reproduction: u8) {
        self.reproduction = reproduction;
    }
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        return (row * self.width + column) as usize;
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        
        return count;
    }

    pub fn get_cells(&self) -> &[Cell] {
        return &self.cells;
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}