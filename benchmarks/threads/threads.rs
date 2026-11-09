/*
Threads: count the steps of the Collatz sequence from every number below
a limit, the numbers shared out among four threads. Plain integer work
spread over the processor's cores, so time on the clock falls as the
cores share it.
*/

#[derive(Default)]
struct Share {
    total: i64,
    longest: i64,
    start: i64,
}

fn steps(mut n: i64) -> i64 {
    let mut count = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        count += 1;
    }
    count
}

fn work(low: i64, high: i64) -> Share {
    let mut share = Share::default();
    for n in low..high {
        let count = steps(n);
        share.total += count;
        if count > share.longest {
            share.longest = count;
            share.start = n;
        }
    }
    share
}

fn main() {
    let limit: i64 = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(100000);
    let parts = 4;
    let workers: Vec<_> = (0..parts)
        .map(|part| {
            let low = 1 + (limit - 1) * part / parts;
            let high = 1 + (limit - 1) * (part + 1) / parts;
            std::thread::spawn(move || work(low, high))
        })
        .collect();
    let mut all = Share::default();
    for worker in workers {
        let share = worker.join().unwrap();
        all.total += share.total;
        if share.longest > all.longest {
            all.longest = share.longest;
            all.start = share.start;
        }
    }
    println!("{} {} {}", all.total, all.start, all.longest);
}
