# life

Runs Conway's Game of Life on a 512 by 512 board whose edges wrap around, from a pseudo-random start, for 50, 200, and 500 generations, then counts the cells still alive. Each generation reads the eight neighbours of every cell, a little over two million reads, and writes the next board.

## What's the same in all three

A byte for each cell, two boards that swap places after every generation, the same order of additions for the neighbours, and the remainder operator to wrap the edges. The start comes from the same generator as the other programs.

## What differs

Swapping the boards: C++ uses `std::swap` and Rust `std::mem::swap`, and Sayon moves the two arrays through a third variable. None of them copies a board.

The sums. C++ adds the bytes as `int`s. Rust and Sayon add them as bytes, and Sayon checks every sum for overflow, as Rust does only in its checked build. Sayon and Rust also check every neighbour's index against the board, which C++ doesn't.

## Results

<!-- results: run.sayon writes this part -->

<!-- end of results -->
