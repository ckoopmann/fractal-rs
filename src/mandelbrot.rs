use num::complex::Complex;

#[derive(Debug)]
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

    pub fn zoom_in(&mut self) -> f64 {
        self.zoom_factor *= 1.1;
        return self.zoom_factor;
    }

    pub fn zoom_out(&mut self) -> f64 {
        self.zoom_factor /= 1.1;
        return self.zoom_factor;
    }

    pub fn move_up(&mut self) -> f64 {
        self.x -= 0.1 / self.zoom_factor;
        return self.x;
    }

    pub fn move_down(&mut self) -> f64 {
        self.x += 0.1 / self.zoom_factor;
        return self.x;
    }

    pub fn move_left(&mut self) -> f64 {
        self.y -= 0.1 / self.zoom_factor;
        return self.y;
    }

    pub fn move_right(&mut self) -> f64 {
        self.y += 0.1 / self.zoom_factor;
        return self.y;
    }
}


// Returns the number of iterations it took the mandelbrot series to diverge
// relative to the total number of iterations
// Return value will be between 0.0 (the original point has already magnitude >2)
// and 1.0 (the series has not diverged within the maximum number of iterations)
// The quotient is smoothed for nicer visualization based on: https://stackoverflow.com/questions/369438/smooth-spectrum-for-mandelbrot-set-rendering
fn mandelbrot_iteration_quotient(x: u32, y: u32, width: u32, height: u32, position: &Position) -> f64 {
    // Convert x / y coordinates to real and imaginary values based on screen position and zoom level
    let real_part = ((x as f64 / width as f64)-1.5)/position.get_zoom_factor() + position.get_x();
    let imaginary_part = ((y as f64 /height as f64)-0.5)/position.get_zoom_factor() + position.get_y();
    let point = Complex::new(real_part, imaginary_part);
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
        let smoothed_iter = iter as f64 + 1.0 - z.norm().ln().ln()/two.ln();
        quotient = smoothed_iter / max_iter as f64;
    }
    return quotient;
}

// Returns a tuple containing the RGB values for the respective pixel based on the iteration quotient
// relative to the total number of iterations
pub fn mandelbrot_rgb_value(x: u32, y: u32, width: u32, height: u32, position: &Position) -> (u8, u8, u8) {
    let quotient = mandelbrot_iteration_quotient(x, y, width, height, position);
    if quotient == 1.0 {
        return (0, 0, 0);
    }
    else if quotient > 0.5 {
        return ((quotient*255.0) as u8, 255, (quotient*255.0) as u8);
    }
    else {
        return (0, (quotient*255.0) as u8, 0);
    }

}

