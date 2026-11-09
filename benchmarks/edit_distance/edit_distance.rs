/*
Edit distance: the fewest one-letter insertions, deletions, and changes that
turn one pseudo-random word of four letters into another, for twenty pairs
of words, by dynamic programming over two rows.
*/

fn word(size: usize, state: &mut i64) -> Vec<u8> {
    let mut letters = Vec::new();
    for _ in 0..size {
        *state = (*state * 1103515245 + 12345) % 2147483648;
        letters.push((*state / 65536 % 4) as u8);
    }
    letters
}

fn distance(first: &[u8], second: &[u8]) -> i64 {
    let mut previous: Vec<i64> = Vec::new();
    let mut current: Vec<i64> = Vec::new();
    for index in 0..=second.len() {
        previous.push(index as i64);
        current.push(0);
    }
    for i in 1..=first.len() {
        current[0] = i as i64;
        for j in 1..=second.len() {
            let cost = if first[i - 1] == second[j - 1] { 0 } else { 1 };
            let mut best = previous[j - 1] + cost;
            if previous[j] + 1 < best {
                best = previous[j] + 1;
            }
            if current[j - 1] + 1 < best {
                best = current[j - 1] + 1;
            }
            current[j] = best;
        }
        std::mem::swap(&mut previous, &mut current);
    }
    previous[second.len()]
}

fn main() {
    let size: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(100);
    let mut state: i64 = 42;
    let mut total = 0;
    for _ in 0..20 {
        let first = word(size, &mut state);
        let second = word(size, &mut state);
        total += distance(&first, &second);
    }
    println!("{total} edits over 20 pairs of {size} letters");
}
