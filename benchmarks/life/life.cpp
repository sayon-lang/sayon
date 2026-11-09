/*
Life: run Conway's Game of Life on a 512 by 512 board whose edges wrap
around, from a pseudo-random start, for a number of generations, then count
the cells alive. A byte for each cell, as Sayon and Rust have.
*/
#include <cstdio>
#include <cstdlib>
#include <utility>
#include <vector>

constexpr long SIZE = 512;

int main(int argc, char** argv) {
    long generations = argc > 1 ? std::atol(argv[1]) : 10;
    std::vector<unsigned char> cells, next;
    long long state = 42;
    for (long cell = 0; cell < SIZE * SIZE; ++cell) {
        state = (state * 1103515245 + 12345) % 2147483648LL;
        cells.push_back(static_cast<unsigned char>(state / 65536 % 2));
        next.push_back(0);
    }
    for (long generation = 0; generation < generations; ++generation) {
        for (long row = 0; row < SIZE; ++row) {
            long above = (row + SIZE - 1) % SIZE * SIZE;
            long here = row * SIZE;
            long below = (row + 1) % SIZE * SIZE;
            for (long column = 0; column < SIZE; ++column) {
                long left = (column + SIZE - 1) % SIZE;
                long right = (column + 1) % SIZE;
                int neighbours = cells[above + left] + cells[above + column] + cells[above + right];
                neighbours += cells[here + left] + cells[here + right];
                neighbours += cells[below + left] + cells[below + column] + cells[below + right];
                unsigned char alive = 0;
                if (neighbours == 3 || (neighbours == 2 && cells[here + column] == 1)) alive = 1;
                next[here + column] = alive;
            }
        }
        std::swap(cells, next);
    }
    long long count = 0;
    for (unsigned char cell : cells) count += cell;
    std::printf("%lld cells alive after %ld generations\n", count, generations);
}
