use num::complex::Complex;

pub struct Position {
    x: f64,
    y: f64,
    zoom_factor: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, zoom_factor: f64) -> Position {
        Position {
            x,
            y,
            zoom_factor,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_zoom_factor(&self) -> f64 {
        self.zoom_factor
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn set_zoom_factor(&mut self, zoom_factor: f64) {
        self.zoom_factor = zoom_factor;
    }

    pub fn zoom_in(&mut self) {
        self.zoom_factor *= 1.1;
    }

    pub fn zoom_out(&mut self) {
        self.zoom_factor /= 1.1;
    }

    pub fn move_left(&mut self) {
        self.x -= 0.1 / self.zoom_factor;
    }

    pub fn move_right(&mut self) {
        self.x += 0.1 / self.zoom_factor;
    }

    pub fn move_up(&mut self) {
        self.y -= 0.1 / self.zoom_factor;
    }

    pub fn move_down(&mut self) {
        self.y += 0.1 / self.zoom_factor;
    }
}


// Returns the number of iterations it took the mandelbrot series to diverge
// relative to the total number of iterations
// Return value will be between 0.0 (the original point has already magnitude >2)
// and 1.0 (the series has not diverged within the maximum number of iterations)
// The quotient is smoothed for nicer visualization based on: https://stackoverflow.com/questions/369438/smooth-spectrum-for-mandelbrot-set-rendering
pub fn mandelbrot_iteration_quotient(x: i32, y: i32, width: i32, height: i32, position: &Position) -> f64 {
    // Convert x / y coordinates to real and imaginary values based on screen position and zoom level
    let realPart = ((x as f64 / width as f64)-1.5)/position.get_zoom_factor() + position.get_x();
    let imaginaryPart = ((y as f64 /height as f64)-0.5)/position.get_zoom_factor() + position.get_y();
    let point = Complex::new(realPart, imaginaryPart);
    let mut z = Complex::new(0.0, 0.0);
    let max_iter = 51;
    let mut iter = 0;
    let two: f64 = 2.0;
    while z.norm() < two && iter < max_iter {
        z = z * z + point;
        iter += 1;
    }
    let mut quotient = 1.0;
    if iter < max_iter {
        let smoothedIter = iter as f64 + 1.0 - z.norm().ln().ln()/two.ln();
        quotient = smoothedIter / max_iter as f64;
    }
    return quotient;
}
