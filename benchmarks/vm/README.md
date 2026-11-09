# vm

Runs a little program on a stack machine: an interpreter that reads one instruction at a time, from an array of numbers, and does what it says. The program adds up i × i, modulo the prime 1,000,000,007, for every i below a million, three million, and ten million, which takes seventeen instructions for each i, or 170 million at the largest input. Interpreters for scripting languages spend their time in a loop like this one.

## What's the same in all three

The same program, written as the same 38 numbers, and the same machine: a loop that looks at the code of the next instruction and picks what to do, a stack of 16 places in an array, and three variables in another. The choice is a `switch` in C++ and a `match` in Rust and Sayon.

## What differs

Naming the codes. C++ names them in an `enum` and Rust as constants, and both use the names in the program and in their `switch` or `match`. Sayon's program and its cases are written as the plain numbers, with a comment saying which number does what.

Sayon checks each array index and each addition and multiplication for overflow. Rust checks the indexes it can't prove are in range, and its arithmetic only in its checked build. C++ checks neither.

## Results

<!-- results: run.sayon writes this part -->

<!-- end of results -->
