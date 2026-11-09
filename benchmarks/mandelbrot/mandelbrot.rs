/*
Mandelbrot: count the points of a size by size grid over the plane that
stay in the Mandelbrot set for 50 steps.
*/

fn inside(cr: f64, ci: f64) -> bool {
    let (mut zr, mut zi) = (0.0f64, 0.0f64);
    for _ in 0..50 {
        let next_r = zr * zr - zi * zi + cr;
        zi = 2.0 * zr * zi + ci;
        zr = next_r;
        if zr * zr + zi * zi > 4.0 {
            return false;
        }
    }
    true
}

fn main() {
    let size: u64 = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(200);
    let mut count = 0u64;
    for y in 0..size {
        let ci = 2.0 * y as f64 / size as f64 - 1.0;
        for x in 0..size {
            let cr = 2.0 * x as f64 / size as f64 - 1.5;
            if inside(cr, ci) {
                count += 1;
            }
        }
    }
    println!("{count} of {} points are in the set", size * size);
}
