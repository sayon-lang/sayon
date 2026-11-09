/*
Hash map: put a lot of integer keys into the standard library's hash
map, look each one up, take every other one out, and look them all up
again. Hashing, probing, and memory for the table.
*/
use std::collections::HashMap;

fn main() {
    let total: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(100000);
    let mut keys: Vec<i64> = Vec::new();
    let mut state: i64 = 42;
    for _ in 0..total {
        state = (state * 1103515245 + 12345) % 2147483648;
        keys.push(state);
    }
    let mut map: HashMap<i64, i64> = HashMap::new();
    for (i, key) in keys.iter().enumerate() {
        map.insert(*key, i as i64);
    }
    let mut sum: i64 = 0;
    for key in &keys {
        sum += map[key];
    }
    for key in keys.iter().step_by(2) {
        map.remove(key);
    }
    let mut found: i64 = 0;
    for key in &keys {
        if map.contains_key(key) {
            found += 1;
        }
    }
    println!("{} {sum} {found}", map.len());
}
