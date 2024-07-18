mod utils;

use fixedbitset::FixedBitSet;
use js_sys::Math;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    /// Size of cells is `width` * `height`
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new_fixed() -> Self {
        let width = 64u32;
        let height = 64u32;
        let size = (width * height) as usize;

        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn new_randomized() -> Self {
        let width = 64u32;
        let height = 64u32;
        let size = (width * height) as usize;

        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, Math::random() >= 0.5);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        return (row * self.width + column) as usize;
    }

    /// The live_neighbor_count method uses deltas and modulo to avoid special casing the edges
    /// of the universe with ifs. When applying a delta of -1, we add self.height - 1 and let
    /// the modulo do its thing, rather than attempting to subtract 1. row and column can be 0,
    /// and if we attempted to subtract 1 from them, there would be an unsigned integer underflow.
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut counter = 0u8;
        for i in [self.height - 1, 0, 1].iter() {
            for j in [self.width - 1, 0, 1].iter() {
                let neighbor_row = (row + i) % self.height;
                let neighbor_col = (column + j) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                counter += match self.cells[idx] {
                    true => 1,
                    false => 0,
                };
            }
        }

        if self.cells.contains(self.get_index(row, column)) {
            counter -= 1;
        }

        return counter;
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let live_neighbors = self.live_neighbor_count(row, col);
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];

                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        (true, n) if n < 2 => false,
                        (true, n) if n > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        self.cells = next;
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
}
