#![feature(async_closure)]
mod utils;

extern crate web_sys;
use futures_channel::oneshot;
use js_sys::Promise;
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use wasm_bindgen::prelude::*;
use web_sys::console;

pub mod mandelbrot;

#[derive(Debug)]
struct SyncUnsafeCell<T> {
    value: UnsafeCell<T>,
}

impl<T> SyncUnsafeCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }
}

unsafe impl<T> Sync for SyncUnsafeCell<T> {}

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
    cells_r: Arc<SyncUnsafeCell<Vec<u8>>>,
    cells_g: Arc<SyncUnsafeCell<Vec<u8>>>,
    cells_b: Arc<SyncUnsafeCell<Vec<u8>>>,
    position: Arc<SyncUnsafeCell<mandelbrot::Position>>,
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn maybe_reverse_range(start: u32, end: u32) -> Box<dyn Iterator<Item = u32>> {
    if start > end {
        Box::new((end..start).rev())
    } else {
        Box::new(start..end)
    }
}

fn get_index(width: u32, row: u32, column: u32) -> usize {
    (row * width + column) as usize
}

fn recalculate_cells(
    row_start: u32,
    row_end: u32,
    col_start: u32,
    col_end: u32,
    position: &mandelbrot::Position,
    cells_r: &mut Vec<u8>,
    cells_g: &mut Vec<u8>,
    cells_b: &mut Vec<u8>,
    width: u32,
    height: u32,
) -> () {
    let col_range = maybe_reverse_range(col_start, col_end);
    col_range.for_each(|col| {
        // Calcualte cells
        let row_range = maybe_reverse_range(row_start, row_end);
        row_range.for_each(|row| {
            // log!("ForEach Col {:?}, Row {:?}", col, row);
            let idx = get_index(width, row, col);
            let (r, g, b) = mandelbrot::mandelbrot_rgb_value(row, col, width, height, position);
            if r == cells_r[idx] && g == cells_g[idx] && b == cells_b[idx] {
                // log!("No change");
            } else {
                // log!("Change");
            }
            cells_r[idx] = r;
            cells_g[idx] = g;
            cells_b[idx] = b;
        });
    });
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

    fn _move_vertical(&self, offset: i64) -> Result<Promise, JsValue> {
        let (tx, rx) = oneshot::channel();
        let position_mutex = self.position.clone();
        let cells_r_mutex = self.cells_r.clone();
        let cells_g_mutex = self.cells_g.clone();
        let cells_b_mutex = self.cells_b.clone();
        let width = self.width.clone();
        let height = self.height.clone();

        thread::spawn(move || {
            let cells_r = &mut cells_r_mutex.get();
            let cells_g = &mut cells_g_mutex.get();
            let cells_b = &mut cells_b_mutex.get();
            let position = &mut position_mutex.get();
            let new_y = position.move_vertical(offset);
            let is_up = offset < 0;

            let start_new = if is_up {
                0
            } else {
                height - offset.abs() as u32
            };

            let end_new = if is_up { offset.abs() as u32 } else { height };

            // Copy cells that don't need to be recalcualted
            let offset_cells = width * offset.abs() as u32;
            let start_index_copy = if is_up { 0 } else { offset_cells } as usize;
            let end_index_copy = if is_up {
                height * width - offset_cells
            } else {
                height * width
            } as usize;
            let target_index_copy = if is_up {
                offset.abs() as u32 * width
            } else {
                0
            } as usize;
            cells_r.copy_within(start_index_copy..end_index_copy, target_index_copy);
            cells_g.copy_within(start_index_copy..end_index_copy, target_index_copy);
            cells_b.copy_within(start_index_copy..end_index_copy, target_index_copy);

            recalculate_cells(
                start_new, end_new, 0, width, position, cells_r, cells_g, cells_b, width, height,
            );
            tx.send(new_y).unwrap();
        });

        let done = async move {
            // console_log!("done!");
            match rx.await {
                Ok(y) => Ok(JsValue::from(y)),
                Err(_) => Err(JsValue::undefined()),
            }
        };
        return Ok(wasm_bindgen_futures::future_to_promise(done));
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn update(&self) {
        let position_mutex = self.position.clone();
        let cells_r_mutex = self.cells_r.clone();
        let cells_g_mutex = self.cells_g.clone();
        let cells_b_mutex = self.cells_b.clone();
        let position = &mut position_mutex.get();
        let cells_r = &mut cells_r_mutex.get();
        let cells_g = &mut cells_g_mutex.get();
        let cells_b = &mut cells_b_mutex.get();
        recalculate_cells(
            0,
            self.height,
            0,
            self.width,
            position,
            cells_r,
            cells_g,
            cells_b,
            self.width,
            self.height,
        );
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
            cells_r: Arc::new(SyncUnsafeCell::new(cells_r)),
            cells_g: Arc::new(SyncUnsafeCell::new(cells_g)),
            cells_b: Arc::new(SyncUnsafeCell::new(cells_b)),
            position: Arc::new(SyncUnsafeCell::new(position)),
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
        log!("cells_r in rust");
        let cells_r_mutex = self.cells_r.clone();
        let cells_r = &mut cells_r_mutex.get();
        log!("cells_r: {:?}", cells_r);
        cells_r.as_ptr()
    }
    pub fn cells_g(&self) -> *const u8 {
        let cells_g_mutex = self.cells_g.clone();
        let cells_g = &mut cells_g_mutex.get();
        cells_g.as_ptr()
    }
    pub fn cells_b(&self) -> *const u8 {
        let cells_b_mutex = self.cells_b.clone();
        let cells_b = &mut cells_b_mutex.get();
        cells_b.as_ptr()
    }

    pub fn zoom_in(self) -> f64 {
        let position_mutex = self.position.clone();
        let position = &mut position_mutex.get();
        return position.zoom_in();
    }

    pub fn zoom_out(self) -> f64 {
        let position_mutex = self.position.clone();
        let position = &mut position_mutex.get();
        return position.zoom_out();
    }

    pub fn move_vertical(&self, offset: i64) -> Result<Promise, JsValue> {
        self._move_vertical(offset)
    }

    pub fn move_horizontal(&self, offset: i64) -> i64 {
        let position_mutex = self.position.clone();
        let position = &mut position_mutex.get();
        let cells_r_mutex = self.cells_r.clone();
        let cells_r = &mut cells_r_mutex.get();
        let cells_g_mutex = self.cells_g.clone();
        let cells_g = &mut cells_g_mutex.get();
        let cells_b_mutex = self.cells_b.clone();
        let cells_b = &mut cells_b_mutex.get();
        let new_x = position.move_horizontal(offset);

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
                    cells_r[idx] = cells_r[idx_source].clone();
                    cells_g[idx] = cells_g[idx_source].clone();
                    cells_b[idx] = cells_b[idx_source].clone();
                }
            }
        } else {
            for col in copy_range {
                let source_col = (col as i64 + offset) as u32;
                for row in 0..self.height {
                    let idx = self.get_index(row, col);
                    let idx_source = self.get_index(row, source_col);
                    cells_r[idx] = cells_r[idx_source].clone();
                    cells_g[idx] = cells_g[idx_source].clone();
                    cells_b[idx] = cells_b[idx_source].clone();
                }
            }
        }

        recalculate_cells(
            0,
            self.height,
            0,
            self.width,
            position,
            cells_r,
            cells_g,
            cells_b,
            self.width,
            self.height,
        );
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
