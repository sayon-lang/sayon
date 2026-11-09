# nqueens

Counts the ways to put n queens on an n by n board so that no two attack each other, on boards of 12 and 13 squares a side. It places a queen in each row in turn, tries every column that is still safe, and backs up when a row has none left, so most of the time goes into recursive calls and checks of three small arrays of flags.

## What's the same in all three

The same search, and a byte for each flag: one array for the columns, and one for each direction of diagonal. `std::vector<unsigned char>` in C++, `Vec<bool>` in Rust, and `Array<boolean>` in Sayon. Each call gets the three arrays to change: by reference in C++, as `&mut` slices in Rust, and as `update` parameters in Sayon.

## What differs

Little. Sayon checks every index against its array, as Rust does where it can't prove the index is in range, and Sayon also checks the sums that make the diagonal numbers for overflow. C++ checks neither. Making the arrays costs nothing worth counting here: C++ and Rust make them at their size in one step, and Sayon pushes one flag at a time, a few dozen of them.

## Results

<!-- results: run.sayon writes this part -->

<!-- end of results -->
