#![feature(portable_simd)]
use image::{ImageBuffer, Rgb};
use std::simd::Simd;
fn calc_simple(x0: f64, y0: f64, i: u32) -> u32 {
    let mut x = 0.0;
    let mut y = 0.0;
    for iteration in 0..i {
        let x2 = x * x;
        let y2 = y * y;

        if x2 + y2 > 4.0 {
            return iteration;
        }

        y = 2.0 * x * y + y0;
        x = x2 - y2 + x0;
    }
    i
}

const LANES: usize = 4;

fn calc_simd(x0: Simd<f64, LANES>, y0: Simd<f64, LANES>, i: u32) -> u32 {
    let mut x = Simd::splat(0.0);
    let mut y = Simd::splat(0.0);

    for iteration in 0..i {
        let x2 = x * x;
        let y2 = y * y;
        let max2: Simd<f64, 4> = x2 + y2;

        if max2.gt(&Simd::splat(4.0)) {
            return iteration;
        }

        y = Simd::splat(2.0) * x * y + y0;
        x = x2 - y2 + x0;
    }
    i
}

fn main() {
    let width = 1920;
    let height = 1080;
    let max = 10;

    let xmax = 0.47;
    let xmin = -2.0;
    let ymax = 1.12;
    let ymin = -1.12;
    //dessa min/max fick jag från kod-delen av mandelbrot wikipedia: https://en.wikipedia.org/wiki/Mandelbrot_set#Basic_properties:~:text=x0%C2%A0%3A%3D%20scaled%20x%20coordinate%20of%20pixel%20(scaled%20to%20lie%20in%20the%20Mandelbrot%20X%20scale%20(%2D2.00%2C%200.47))%0A%20%20%20%20y0%C2%A0%3A%3D%20scaled%20y%20coordinate%20of%20pixel%20(scaled%20to%20lie%20in%20the%20Mandelbrot%20Y%20scale%20(%2D1.12%2C%201.12))

    let mut img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let xp = xmin + (x as f64 / width as f64) * (xmax - xmin);
        let yp = ymin + (y as f64 / height as f64) * (ymax - ymin);

        let iteration = calc_simple(xp, yp, max);

        let color = if iteration == max {
            0
        } else {
            (255 * iteration / max) as u8
        };

        *pixel = Rgb([color, color, color]);
    }

    img.save("mandelbrot.png").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
}
