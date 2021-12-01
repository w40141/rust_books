use std::time::Duration;

use num::complex::Complex;
use std::ops::Add;

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at)
        }
        rows.push(row);
    }
    rows
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);
    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    return max_iters;
}

fn render_mandelbrot(escaped_vals: Vec<Vec<usize>>) {
    for row in escaped_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                2..=5 => '.',
                5..=10 => '?',
                10..=30 => '*',
                30..=100 => '+',
                100..=200 => 'x',
                200..=400 => '$',
                400..=700 => '#',
                _ => '%',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}

fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b);
    print!("{}", res);
    let durations = add(Duration::new(5, 0), Duration::new(10, 0));
    println!("{:?}", durations);
    // let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 80, 24);
    // render_mandelbrot(mandelbrot);
}
