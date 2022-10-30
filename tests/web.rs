//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate fractal_rs;
use fractal_rs::Universe;
use fractal_rs::mandelbrot::{Position, mandelbrot_rgb_value};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_universe() {
    let universe = Universe::new(512, 512, 0, 0);
}

#[wasm_bindgen_test]
pub fn test_mandelbrot_rgb() {
    let position = Position::new(0.0, 0.0, 1.0);
    let quotient = mandelbrot_rgb_value(0, 0, 10, 10, &position);
    assert!(quotient.0 <= 255);
    assert!(quotient.1 <= 255);
    assert!(quotient.2 <= 255);
}

