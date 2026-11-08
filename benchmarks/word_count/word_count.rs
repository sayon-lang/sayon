// Word count: make up a long text from a small set of syllables, count how
// often each word comes up, and list the most common.
use std::collections::HashMap;

fn main() {
    let total: u64 = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(100000);
    let parts = ["ka", "lo", "mi", "ne", "ru", "sa", "ti", "vo", "ze", "pa"];
    let mut counts: HashMap<String, u64> = HashMap::new();
    let mut state: u64 = 42;
    for _ in 0..total {
        state = (state * 1103515245 + 12345) % 2147483648;
        let mut word = String::new();
        for _ in 0..1 + state / 65536 % 3 {
            state = (state * 1103515245 + 12345) % 2147483648;
            word.push_str(parts[(state / 65536 % 10) as usize]);
        }
        *counts.entry(word).or_insert(0) += 1;
    }
    let mut found: Vec<(&String, &u64)> = counts.iter().collect();
    found.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    println!("{} different words", counts.len());
    for (word, count) in found.iter().take(5) {
        println!("{word} {count}");
    }
}
