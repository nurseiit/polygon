use core::fmt;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
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

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                next_cells[idx] = match (cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (rest, _) => rest,
                }
            }
        }
        self.cells = next_cells;
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        [self.height - 1, 0, 1]
            .iter()
            .map(|&dx| {
                [self.width - 1, 0, 1]
                    .into_iter()
                    .filter(|&dy| !(dx == 0 && dy == 0))
                    .map(|dy| ((dx + row) % self.height, (dy + col) % self.width))
                    .map(|(n_row, n_col)| self.get_index(n_row, n_col))
                    .filter(|&idx| self.cells[idx] == Cell::Alive)
                    .count() as u8
            })
            .sum()
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                write!(
                    f,
                    "{}",
                    match cell {
                        Cell::Dead => '◻',
                        Cell::Alive => '◼',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
