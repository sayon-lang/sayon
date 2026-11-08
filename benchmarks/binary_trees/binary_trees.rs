// Binary trees: build many complete binary trees, walk each one, and let
// it go, while one long-lived tree stays.

struct Tree {
    children: Option<(Box<Tree>, Box<Tree>)>,
}

fn make(depth: u32) -> Box<Tree> {
    if depth == 0 {
        return Box::new(Tree { children: None });
    }
    Box::new(Tree { children: Some((make(depth - 1), make(depth - 1))) })
}

fn check(tree: &Tree) -> u64 {
    match &tree.children {
        Some((left, right)) => 1 + check(left) + check(right),
        None => 1,
    }
}

fn main() {
    let most: u32 = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(10).max(6);
    let stretch = most + 1;
    println!("stretch tree of depth {stretch}\t check: {}", check(&make(stretch)));
    let long_lived = make(most);
    for depth in (4..=most).step_by(2) {
        let iterations = 1u64 << (most - depth + 4);
        let total: u64 = (0..iterations).map(|_| check(&make(depth))).sum();
        println!("{iterations}\t trees of depth {depth}\t check: {total}");
    }
    println!("long lived tree of depth {most}\t check: {}", check(&long_lived));
}
