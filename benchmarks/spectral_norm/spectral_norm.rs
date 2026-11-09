/*
Spectral norm: the largest singular value of an infinite matrix, worked
out with the power method on an n by n corner of it.
*/

fn entry(i: usize, j: usize) -> f64 {
    1.0 / ((i + j) * (i + j + 1) / 2 + i + 1) as f64
}

fn times(v: &[f64], out: &mut [f64]) {
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = v.iter().enumerate().map(|(j, x)| entry(i, j) * x).sum();
    }
}

fn times_transposed(v: &[f64], out: &mut [f64]) {
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = v.iter().enumerate().map(|(j, x)| entry(j, i) * x).sum();
    }
}

fn times_both(v: &[f64], out: &mut [f64], between: &mut [f64]) {
    times(v, between);
    times_transposed(between, out);
}

fn main() {
    let n: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(100);
    let mut u = vec![1.0; n];
    let mut v = vec![0.0; n];
    let mut between = vec![0.0; n];
    for _ in 0..10 {
        times_both(&u, &mut v, &mut between);
        times_both(&v, &mut u, &mut between);
    }
    let vbv: f64 = u.iter().zip(&v).map(|(a, b)| a * b).sum();
    let vv: f64 = v.iter().map(|b| b * b).sum();
    println!("{:.9}", (vbv / vv).sqrt());
}
