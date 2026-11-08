// Fannkuch-redux: flip a stack of pancakes, numbered 1 to n, for every
// order they can start in, and count the most flips any order needs.

fn main() {
    let n: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(10);
    let mut perm = vec![0usize; n];
    let mut perm1: Vec<usize> = (0..n).collect();
    let mut count = vec![0usize; n];
    let mut checksum: i64 = 0;
    let mut most = 0;
    let mut index = 0;
    let mut r = n;
    loop {
        while r != 1 {
            count[r - 1] = r;
            r -= 1;
        }
        perm.copy_from_slice(&perm1);
        let mut flips = 0;
        let mut k = perm[0];
        while k != 0 {
            perm[..=k].reverse();
            flips += 1;
            k = perm[0];
        }
        most = most.max(flips);
        checksum += if index % 2 == 0 { flips } else { -flips };
        loop {
            if r == n {
                println!("{checksum}\nPfannkuchen({n}) = {most}");
                return;
            }
            let first = perm1[0];
            for i in 0..r {
                perm1[i] = perm1[i + 1];
            }
            perm1[r] = first;
            count[r] -= 1;
            if count[r] > 0 {
                break;
            }
            r += 1;
        }
        index += 1;
    }
}
