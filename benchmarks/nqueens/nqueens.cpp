/*
N-queens: count the ways to put n queens on an n by n board so that none
attacks another, placing one queen a row at a time and backing up when a row
has no safe square left. A byte for each flag, as Sayon and Rust have.
*/
#include <cstdio>
#include <cstdlib>
#include <vector>

using Flags = std::vector<unsigned char>;

static long long place(int row, int size, Flags& columns, Flags& rising, Flags& falling) {
    if (row == size) return 1;
    long long count = 0;
    for (int column = 0; column < size; ++column) {
        int up = row + column;
        int down = row - column + size - 1;
        if (!columns[column] && !rising[up] && !falling[down]) {
            columns[column] = rising[up] = falling[down] = 1;
            count += place(row + 1, size, columns, rising, falling);
            columns[column] = rising[up] = falling[down] = 0;
        }
    }
    return count;
}

int main(int argc, char** argv) {
    int size = argc > 1 ? std::atoi(argv[1]) : 8;
    Flags columns(size), rising(2 * size - 1), falling(2 * size - 1);
    std::printf("%lld solutions for %d queens\n", place(0, size, columns, rising, falling), size);
}
