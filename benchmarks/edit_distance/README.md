# edit_distance

Works out the edit distance, the fewest one-letter insertions, deletions, and changes that turn one word into another, for twenty pairs of pseudo-random words over a four-letter alphabet. The words are 500, 1,000, and 2,000 letters long, so the largest input fills in 80 million cells of the classic table.

## What's the same in all three

The same dynamic programming over two rows of 64-bit integers, the row above and the row being filled in, which swap places after each row. The letters are bytes, and the words come from the same generator as the other programs, a byte for each letter.

## What differs

Swapping the rows: C++ uses `std::swap` and Rust `std::mem::swap`, and Sayon moves the two arrays through a third variable. None of them copies a row.

Sayon checks every index and every addition. Rust checks indexes it can't prove are in range, and its additions only in its checked build. C++ checks neither.

## Results

<!-- results: run.sayon writes this part -->

<!-- end of results -->
