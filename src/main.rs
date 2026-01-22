#![feature(portable_simd)]
#![feature(test)]
extern crate test;
use image::{ImageBuffer, Rgb};
use std::simd::{Mask, Simd, cmp::SimdPartialOrd};

fn calc_simple(x0: f32, y0: f32, i: u32) -> u32 {
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

const LANES: usize = 16;

#[inline(always)]
fn calc_simd(x0: Simd<f32, LANES>, y0: Simd<f32, LANES>, max: u32) -> Simd<u32, LANES> {
    let mut x: Simd<f32, LANES> = Simd::splat(0.0);
    let mut y: Simd<f32, LANES> = Simd::splat(0.0);
    let mut iteration = Simd::splat(0u32);
    let mut active = Mask::<_, LANES>::splat(true);

    for _ in 0..max {
        let x2: Simd<f32, LANES> = x * x;
        let y2: Simd<f32, LANES> = y * y;
        let max2: Simd<f32, LANES> = x2 + y2;

        let escaped = max2.simd_gt(Simd::splat(4.0));
        active &= !escaped;

        if !active.any() {
            break;
        }

        iteration += active.select(Simd::splat(1), Simd::splat(0));
        y = Simd::splat(2.0) * x * y + y0;
        x = x2 - y2 + x0;
    }
    iteration
}

fn simple_image(width: usize, height: usize, xmax: f32, xmin: f32, ymax: f32, ymin: f32, max: u32) {
    let mut img = ImageBuffer::new(width as u32, height as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let xp = xmin + (x as f32 / width as f32) * (xmax - xmin);
        let yp = ymin + (y as f32 / height as f32) * (ymax - ymin);

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

fn simd_image(width: usize, height: usize, xmax: f32, xmin: f32, ymax: f32, ymin: f32, max: u32) {
    let mut img = ImageBuffer::new(width as u32, height as u32);

    let dx = (xmax - xmin) / width as f32;
    let dy = (ymax - ymin) / height as f32;

    for yp in 0..height {
        let y = ymin + yp as f32 * dy;

        for xp in (0..width).step_by(LANES) {
            let mut xvals = [0.0; LANES];
            for i in 0..LANES {
                if xp + i < width {
                    xvals[i] = xmin + (xp + i) as f32 * dx;
                }
            }

            let xout = Simd::from_array(xvals);
            let yout = Simd::splat(y);

            let iterations = calc_simd(xout, yout, max);
            let iterations = iterations.to_array();

            for i in 0..LANES {
                if xp + i < width {
                    let val = (255 * iterations[i] / max as u32) as u8;
                    img.put_pixel((xp + i) as u32, yp as u32, Rgb([val, val, val]));
                }
            }
        }
    }
    img.save("mandelbrot_simd.png").unwrap();
}
fn main() {
    let width: usize = 1920;
    let height: usize = 1080;
    let max = 255;

    let xmax: f32 = 0.47;
    let xmin: f32 = -2.0;
    let ymax: f32 = 1.12;
    let ymin: f32 = -1.12;

    simple_image(width, height, xmax, xmin, ymax, ymin, max);
    simd_image(width, height, xmax, xmin, ymax, ymin, max);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_simple_calc(b: &mut Bencher) {
        b.iter(|| {
            for x in 500..516 {
                calc_simple(x as f32, 500.0, 255);
            }
        });
    }

    #[bench]
    fn bench_simd_calc(b: &mut Bencher) {
        let array = core::array::from_fn(|i| 500.0 + i as f32);
        let x0 = Simd::from_array(array);
        let y0 = Simd::splat(500.0);
        b.iter(|| calc_simd(x0, y0, 255));
    }

    #[bench]
    fn bench_image_simple(b: &mut Bencher) {
        let width: usize = 640;
        let height: usize = 360;
        let max = 64;

        let xmax = 0.47;
        let xmin = -2.0;
        let ymax = 1.12;
        let ymin = -1.12;
        b.iter(|| simple_image(width, height, xmax, xmin, ymax, ymin, max));
    }
    #[bench]
    fn bench_image_simd(b: &mut Bencher) {
        let width: usize = 640;
        let height: usize = 360;
        let max = 64;

        let xmax = 0.47;
        let xmin = -2.0;
        let ymax = 1.12;
        let ymin = -1.12;
        b.iter(|| simd_image(width, height, xmax, xmin, ymax, ymin, max));
    }
}
