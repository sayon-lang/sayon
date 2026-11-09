# dijkstra

Finds the shortest distance from one place to every other on a pseudo-random map of 100,000, 300,000, and a million places, each with four one-way roads to random places, with Dijkstra's algorithm. It prints how many places it reached, their distances added up, and the farthest. The work is building the map, an array of roads for every place, and running a binary heap that holds the places to visit next.

## What's the same in all three

The same map, the same algorithm, and the same heap: a binary heap kept in an array, written out the same way in all three, rather than `std::priority_queue` in C++ and `BinaryHeap` in Rust, which Sayon doesn't have. A place can go on the heap more than once, and is skipped when it comes off again after its distance is known. Each road is a small structure of two 64-bit integers.

## What differs

Making the road lists: C++ makes a million empty `std::vector`s at once, and Rust and Sayon push an empty array for each place. Each place's list then grows as its four roads are added, in all three.

Sayon checks the index of every array access and every addition. Rust checks indexes it can't prove are in range, and additions only in its checked build. C++ checks neither.

## Results

<!-- results: run.sayon writes this part -->

<!-- end of results -->
