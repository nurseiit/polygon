use core::fmt;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

extern crate js_sys;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let size = (height * width) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        (0..size).for_each(|i| {
            cells.set(i, js_sys::Math::random() > 0.5);
        });

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const usize {
        self.cells.as_slice().as_ptr()
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

                next_cells.set(
                    idx,
                    match (cell, live_neighbours) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (rest, _) => rest,
                    },
                );
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
                    .filter(|&idx| self.cells[idx])
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
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                write!(
                    f,
                    "{}",
                    match cell {
                        false => '◻',
                        true => '◼',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
