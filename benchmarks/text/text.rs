/*
Text: format many lines into one string, split it back into an array of
lines, and look through them, counting characters as Sayon's `length`
does.
*/

fn main() {
    let total: u64 = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(100000);
    let mut lines: Vec<String> = Vec::new();
    let mut state: u64 = 42;
    for i in 0..total {
        state = (state * 1103515245 + 12345) % 2147483648;
        lines.push(format!("item {i}: {}", state / 65536));
    }
    let text = lines.join("\n");
    let pieces: Vec<String> = text.split('\n').map(String::from).collect();
    let mut sevens = 0;
    let mut characters = 0;
    for line in &pieces {
        characters += line.chars().count();
        if line.contains('7') {
            sevens += 1;
        }
    }
    println!("{} characters, {characters} in lines, {sevens} lines with a 7", text.chars().count());
}
