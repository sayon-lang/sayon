/*
N-queens: count the ways to put n queens on an n by n board so that none
attacks another, placing one queen a row at a time and backing up when a row
has no safe square left. Recursion, and three small arrays of flags.
*/

fn place(row: usize, size: usize, columns: &mut [bool], rising: &mut [bool], falling: &mut [bool]) -> u64 {
    if row == size {
        return 1;
    }
    let mut count = 0;
    for column in 0..size {
        let up = row + column;
        let down = row + size - 1 - column;
        if !columns[column] && !rising[up] && !falling[down] {
            columns[column] = true;
            rising[up] = true;
            falling[down] = true;
            count += place(row + 1, size, columns, rising, falling);
            columns[column] = false;
            rising[up] = false;
            falling[down] = false;
        }
    }
    count
}

fn main() {
    let size: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(8);
    let mut columns = vec![false; size];
    let mut rising = vec![false; 2 * size - 1];
    let mut falling = vec![false; 2 * size - 1];
    println!("{} solutions for {size} queens", place(0, size, &mut columns, &mut rising, &mut falling));
}
