// Sort: fill an array with pseudo-random integers and sort it with the
// standard library's stable sort, as Sayon's sort is stable.

fn main() {
    let total: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(100000);
    let mut numbers: Vec<i64> = Vec::new();
    let mut state: i64 = 42;
    for _ in 0..total {
        state = (state * 1103515245 + 12345) % 2147483648;
        numbers.push(state);
    }
    numbers.sort();
    let checksum: i64 = numbers.iter().step_by(1000).sum();
    println!("{} {} {} {checksum}", numbers[0], numbers[total / 2], numbers[total - 1]);
}
