mod utils;

extern crate web_sys;
use web_sys::console;
use wasm_bindgen::prelude::*;

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

// Interanl functions NOT exposed to JS
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn update(&mut self) {
        let mut next_r = self.cells_r.clone();
        let mut next_g = self.cells_g.clone();
        let mut next_b = self.cells_b.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let (r, g, b) = mandelbrot::mandelbrot_rgb_value(row, col, self.width, self.height, &self.position);
                next_r[idx] = r;
                next_g[idx] = g;
                next_b[idx] = b;
            }
        }
        self.cells_r = next_r;
        self.cells_g = next_g;
        self.cells_b = next_b;
    }

    pub fn new(width: u32, height: u32, x: f64, y: f64, zoom: f64) -> Universe {
        utils::set_panic_hook();
        let position = mandelbrot::Position::new(x, y, zoom);

        let mut cells_r = Vec::with_capacity((width * height) as usize);
        cells_r.resize((width * height) as usize, 0);
        let mut cells_g = Vec::with_capacity((width * height) as usize);
        cells_g.resize((width * height) as usize, 0);
        let mut cells_b = Vec::with_capacity((width * height) as usize);
        cells_b.resize((width * height) as usize, 0);

        for row in 0..height {
            for col in 0..width {
                let idx = (row * width + col) as usize;
                let (r, g, b) = mandelbrot::mandelbrot_rgb_value(row, col, width, height, &position);
                cells_r[idx] = r;
                cells_g[idx] = g;
                cells_b[idx] = b;
            }
        }

        let universe = Universe {
            width,
            height,
            cells_r,
            cells_g,
            cells_b,
            position,
        };
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
    pub  fn cells_g(&self) -> *const u8 {
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

    pub fn move_left(&mut self) -> f64 {
        return self.position.move_left();
    }

    pub fn move_right(&mut self) -> f64 {
        return self.position.move_right();
    }

    pub fn move_up(&mut self) -> f64 {
        return self.position.move_up();
    }

    pub fn move_down(&mut self) -> f64 {
        return self.position.move_down();
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
