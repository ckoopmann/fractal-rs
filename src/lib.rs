mod utils;

extern crate web_sys;
use wasm_bindgen::prelude::*;
use web_sys::console;

pub mod mandelbrot;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Universe {
    width: u32,
    height: u32,
    cells_r: Vec<u8>,
    cells_g: Vec<u8>,
    cells_b: Vec<u8>,
    position: mandelbrot::Position,
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn maybe_reverse_range(start: u32, end: u32) -> Box<dyn Iterator<Item=u32>> {
    if start > end {
        Box::new((end..start).rev())
    } else {
        Box::new(start..end)
    }
}

// fn maybe_reverse_range<usize>(start: usize, end: usize) -> Box<dyn Iterator<Item=usize:std::iter>> 
// {
//     if start > end {
//         Box::new((end..start).rev())
//     } else {
//         Box::new((start..end))
//     }
// }

// Interanl functions NOT exposed to JS
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn recalculate_cells(&mut self, row_start: u32, row_end: u32, col_start: u32, col_end: u32) -> ()
    {
        let col_range = maybe_reverse_range(col_start, col_end);
        col_range.for_each(|col| {
            // Calcualte cells
            let row_range = maybe_reverse_range(row_start, row_end);
            row_range.for_each(|row| {
                // log!("ForEach Col {:?}, Row {:?}", col, row);
                let idx = self.get_index(row, col);
                let (r, g, b) = mandelbrot::mandelbrot_rgb_value(
                    row,
                    col,
                    self.width,
                    self.height,
                    &self.position,
                );
                if r == self.cells_r[idx] && g == self.cells_g[idx] && b == self.cells_b[idx] {
                    // log!("No change");
                } else {
                    // log!("Change");
                }
                self.cells_r[idx] = r;
                self.cells_g[idx] = g;
                self.cells_b[idx] = b;
            });
        });
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn update(&mut self) {
        self.recalculate_cells(0, self.height, 0, self.width);
    }

    pub fn new(width: u32, height: u32, x: i64, y: i64, zoom: f64) -> Universe {
        utils::set_panic_hook();
        let position = mandelbrot::Position::new(x, y, zoom);

        let mut cells_r = Vec::with_capacity((width * height) as usize);
        cells_r.resize((width * height) as usize, 0);
        let mut cells_g = Vec::with_capacity((width * height) as usize);
        cells_g.resize((width * height) as usize, 0);
        let mut cells_b = Vec::with_capacity((width * height) as usize);
        cells_b.resize((width * height) as usize, 0);


        let mut universe = Universe {
            width,
            height,
            cells_r,
            cells_g,
            cells_b,
            position,
        };
        universe.update();
        return universe;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells_r(&self) -> *const u8 {
        self.cells_r.as_ptr()
    }
    pub fn cells_g(&self) -> *const u8 {
        self.cells_g.as_ptr()
    }
    pub fn cells_b(&self) -> *const u8 {
        self.cells_b.as_ptr()
    }

    pub fn zoom_in(&mut self) -> f64 {
        return self.position.zoom_in();
    }

    pub fn zoom_out(&mut self) -> f64 {
        return self.position.zoom_out();
    }

    pub fn move_vertical(&mut self, offset: i64) -> i64 {
        let new_y = self.position.move_vertical(offset);

        let is_up = offset < 0;

        let start_new = if is_up {
            0
        } else {
            self.height - offset.abs() as u32
        };

        let end_new = if is_up {
            offset.abs() as u32
        } else {
            self.height
        };

        // Copy cells that don't need to be recalcualted
        let offset_cells = self.width * offset.abs() as u32;
        let start_index_copy = if is_up { 0 } else { offset_cells } as usize;
        let end_index_copy = if is_up {
            self.height * self.width - offset_cells
        } else {
            self.height * self.width
        } as usize;
        let target_index_copy = if is_up {
            offset.abs() as u32 * self.width
        } else {
            0
        } as usize;
        self.cells_r
            .copy_within(start_index_copy..end_index_copy, target_index_copy);
        self.cells_g
            .copy_within(start_index_copy..end_index_copy, target_index_copy);
        self.cells_b
            .copy_within(start_index_copy..end_index_copy, target_index_copy);

        self.recalculate_cells(start_new, end_new, 0, self.width);

        return new_y;
    }

    pub fn move_horizontal(&mut self, offset: i64) -> i64 {
        let new_x = self.position.move_horizontal(offset);

        let is_left = offset < 0;

        let col_start_new = if is_left {
            0
        } else {
            self.width - offset.abs() as u32
        };

        let col_end_new = if is_left {
            offset.abs() as u32
        } else {
            self.width
        };

        let copy_range = if is_left {
            col_end_new..self.width
        } else {
            0..col_start_new
        };
        if is_left {
            for col in copy_range.rev() {
                let source_col = (col as i64 + offset) as u32;
                for row in 0..self.height {
                    let idx = self.get_index(row, col);
                    let idx_source = self.get_index(row, source_col);
                    self.cells_r[idx] = self.cells_r[idx_source].clone();
                    self.cells_g[idx] = self.cells_g[idx_source].clone();
                    self.cells_b[idx] = self.cells_b[idx_source].clone();
                }
            }
        } else {
            for col in copy_range {
                let source_col = (col as i64 + offset) as u32;
                for row in 0..self.height {
                    let idx = self.get_index(row, col);
                    let idx_source = self.get_index(row, source_col);
                    self.cells_r[idx] = self.cells_r[idx_source].clone();
                    self.cells_g[idx] = self.cells_g[idx_source].clone();
                    self.cells_b[idx] = self.cells_b[idx_source].clone();
                }
            }
        }

        self.recalculate_cells(0, self.height, col_start_new, col_end_new);
        return new_x;
    }
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}
