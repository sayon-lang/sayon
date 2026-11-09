/*!
Sayon's allocator counts what a program holds while the sandbox limits
it. This is the only test in its process, so that nothing else allocates
or frees while it counts, but the test harness, a little.
*/

use sayon_runtime::alloc;

/** Never called: the limit is far above anything the test holds. */
fn over_the_limit() -> ! {
    std::process::abort()
}

#[test]
fn what_is_held_is_counted() {
    alloc::limit(usize::MAX, over_the_limit);
    let before = alloc::held();
    let large = std::hint::black_box(vec![1u8; 1 << 20]);
    let holding = alloc::held();
    assert!(
        holding - before >= 1 << 20,
        "a block of 1 MiB counted as {} bytes",
        holding - before
    );
    let small: Vec<Box<[u8; 40]>> = (0..1000).map(|_| Box::new([2; 40])).collect();
    assert!(
        alloc::held() - holding >= 40_000,
        "a thousand blocks of 40 bytes counted as {} bytes",
        alloc::held() - holding
    );
    drop(small);
    drop(large);
    assert!(
        alloc::held() - before < 1 << 16,
        "{} bytes still counted after freeing",
        alloc::held() - before
    );
    alloc::unlimit();
}
