/*
Binary trees: build many complete binary trees, walk each one, and let
it go, while one long-lived tree stays.
*/
#include <cstdio>
#include <cstdlib>
#include <memory>

struct Tree {
    std::unique_ptr<Tree> left, right;
};

static std::unique_ptr<Tree> make(int depth) {
    auto tree = std::make_unique<Tree>();
    if (depth > 0) {
        tree->left = make(depth - 1);
        tree->right = make(depth - 1);
    }
    return tree;
}

static long check(const Tree& tree) {
    long count = 1;
    if (tree.left) count += check(*tree.left);
    if (tree.right) count += check(*tree.right);
    return count;
}

int main(int argc, char** argv) {
    int most = argc > 1 ? std::atoi(argv[1]) : 10;
    if (most < 6) most = 6;
    int stretch = most + 1;
    std::printf("stretch tree of depth %d\t check: %ld\n", stretch, check(*make(stretch)));
    auto long_lived = make(most);
    for (int depth = 4; depth <= most; depth += 2) {
        long iterations = 1L << (most - depth + 4);
        long total = 0;
        for (long round = 0; round < iterations; ++round) total += check(*make(depth));
        std::printf("%ld\t trees of depth %d\t check: %ld\n", iterations, depth, total);
    }
    std::printf("long lived tree of depth %d\t check: %ld\n", most, check(*long_lived));
}
