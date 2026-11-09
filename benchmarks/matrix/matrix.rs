/*
Matrix: multiply two square matrices of floating point numbers, each
kept row by row in one array. Three nested loops over arrays, the kind
of work compilers like to turn into vector instructions.
*/

fn multiply(a: &[f64], b: &[f64], c: &mut [f64], n: usize) {
    for i in 0..n {
        for k in 0..n {
            let factor = a[i * n + k];
            for j in 0..n {
                c[i * n + j] += factor * b[k * n + j];
            }
        }
    }
}

fn main() {
    let n: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(200);
    let mut a: Vec<f64> = Vec::new();
    let mut b: Vec<f64> = Vec::new();
    let mut c: Vec<f64> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            a.push(((i + j) % 10) as f64 / 10.0);
            b.push(((i * j) % 7) as f64 / 7.0);
            c.push(0.0);
        }
    }
    multiply(&a, &b, &mut c, n);
    let mut total = 0.0;
    for value in &c {
        total += value;
    }
    println!("{:.6} {:.6}", total, c[(n / 2) * n + n / 3]);
}
