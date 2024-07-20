#[macro_use]
mod utils;

use std::u32;

use fixedbitset::FixedBitSet;
use js_sys::Math;
use utils::Timer;
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
    /// Initial state of cells
    init_states: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    /// Constructor for a new Universe with the given height and width.
    /// All cells are uninitialized. Set cell values through `set_cells()`
    pub fn new(height: u32, width: u32) -> Self {
        // make a error panic message more informative
        utils::set_panic_hook();

        let cells = FixedBitSet::with_capacity((width * height) as usize);
        let init_states = cells.clone();
        Universe {
            width,
            height,
            cells,
            init_states,
        }
    }

    pub fn new_fixed() -> Self {
        let width = 64u32;
        let height = 64u32;
        let size = (width * height) as usize;

        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }
        let init_states = cells.clone();

        Universe {
            width,
            height,
            cells,
            init_states,
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
        let init_states = cells.clone();

        Universe {
            width,
            height,
            cells,
            init_states,
        }
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.reset_cells();
    }

    /// Set the height of the universes.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.reset_cells();
    }

    /// Reset all cells to DEAD
    pub fn reset_cells(&mut self) {
        let size = (self.width * self.height) as usize;
        self.cells.set_range(0..size, false);
    }

    /// Reset to the initial state
    pub fn reset_init_state(&mut self) {
        self.cells = self.init_states.clone();
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        return (row * self.width + column) as usize;
    }

    /// The live_neighbor_count method uses deltas and modulo to avoid special casing the edges
    /// of the universe with ifs. When applying a delta of -1, we add self.height - 1 and let
    /// the modulo do its thing, rather than attempting to subtract 1. row and column can be 0,
    /// and if we attempted to subtract 1 from them, there would be an unsigned integer underflow.
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
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

                // if next[idx] != cell {
                //     log!(
                //         "cell at ({},{}) changes from {} to {}",
                //         row,
                //         col,
                //         cell,
                //         next[idx],
                //     );
                // }
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

    pub fn init_states(&self) -> *const usize {
        self.init_states.as_slice().as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);
    }
}

/// No WASM bindgen. No expose to JavaScript
impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}
