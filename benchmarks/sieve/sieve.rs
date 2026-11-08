// Sieve: count the primes below a limit with the sieve of Eratosthenes,
// one byte for each number. A long walk through memory in simple steps.

fn main() {
    let limit: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(1000000);
    let mut composite = vec![false; limit];
    let mut count: i64 = 0;
    let mut last: usize = 0;
    for n in 2..limit {
        if composite[n] {
            continue;
        }
        count += 1;
        last = n;
        let mut multiple = n * n;
        while multiple < limit {
            composite[multiple] = true;
            multiple += n;
        }
    }
    println!("{count} {last}");
}
