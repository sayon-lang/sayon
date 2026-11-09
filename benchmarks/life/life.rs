/*
Life: run Conway's Game of Life on a 512 by 512 board whose edges wrap
around, from a pseudo-random start, for a number of generations, then count
the cells alive. A grid walked in nested loops, a byte for each cell.
*/

const SIZE: usize = 512;

fn main() {
    let generations: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(10);
    let mut cells: Vec<u8> = Vec::new();
    let mut next: Vec<u8> = Vec::new();
    let mut state: i64 = 42;
    for _ in 0..SIZE * SIZE {
        state = (state * 1103515245 + 12345) % 2147483648;
        cells.push((state / 65536 % 2) as u8);
        next.push(0);
    }
    for _ in 0..generations {
        for row in 0..SIZE {
            let above = (row + SIZE - 1) % SIZE * SIZE;
            let here = row * SIZE;
            let below = (row + 1) % SIZE * SIZE;
            for column in 0..SIZE {
                let left = (column + SIZE - 1) % SIZE;
                let right = (column + 1) % SIZE;
                let mut neighbours = cells[above + left] + cells[above + column] + cells[above + right];
                neighbours += cells[here + left] + cells[here + right];
                neighbours += cells[below + left] + cells[below + column] + cells[below + right];
                let mut alive = 0;
                if neighbours == 3 || (neighbours == 2 && cells[here + column] == 1) {
                    alive = 1;
                }
                next[here + column] = alive;
            }
        }
        std::mem::swap(&mut cells, &mut next);
    }
    let count: u64 = cells.iter().map(|&cell| cell as u64).sum();
    println!("{count} cells alive after {generations} generations");
}
