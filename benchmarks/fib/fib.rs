/*
Fib: the Fibonacci numbers worked out the slow way, by a function that
calls itself twice for each number. Nothing but calls, comparisons, and
additions, so it shows what a function call costs.
*/

fn fib(n: i64) -> i64 {
    if n < 2 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    let n: i64 = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(30);
    println!("{}", fib(n));
}
