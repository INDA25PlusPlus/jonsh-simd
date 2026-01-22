use image::{ImageBuffer, Rgb};
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

fn main() {
    let width = 3840;
    let height = 2160;
    let max = 100;

    let xmax = 2.0;
    let xmin = -2.0;
    let ymax = 2.0;
    let ymin = -2.0;

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
