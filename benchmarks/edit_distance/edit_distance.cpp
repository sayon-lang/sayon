/*
Edit distance: the fewest one-letter insertions, deletions, and changes that
turn one pseudo-random word of four letters into another, for twenty pairs
of words, by dynamic programming over two rows.
*/
#include <cstdio>
#include <cstdlib>
#include <utility>
#include <vector>

static std::vector<unsigned char> word(long size, long long& state) {
    std::vector<unsigned char> letters;
    for (long letter = 0; letter < size; ++letter) {
        state = (state * 1103515245 + 12345) % 2147483648LL;
        letters.push_back(static_cast<unsigned char>(state / 65536 % 4));
    }
    return letters;
}

static long long distance(const std::vector<unsigned char>& first, const std::vector<unsigned char>& second) {
    std::vector<long long> previous, current;
    for (size_t index = 0; index <= second.size(); ++index) {
        previous.push_back(static_cast<long long>(index));
        current.push_back(0);
    }
    for (size_t i = 1; i <= first.size(); ++i) {
        current[0] = static_cast<long long>(i);
        for (size_t j = 1; j <= second.size(); ++j) {
            long long cost = first[i - 1] == second[j - 1] ? 0 : 1;
            long long best = previous[j - 1] + cost;
            if (previous[j] + 1 < best) best = previous[j] + 1;
            if (current[j - 1] + 1 < best) best = current[j - 1] + 1;
            current[j] = best;
        }
        std::swap(previous, current);
    }
    return previous[second.size()];
}

int main(int argc, char** argv) {
    long size = argc > 1 ? std::atol(argv[1]) : 100;
    long long state = 42;
    long long total = 0;
    for (int pair = 0; pair < 20; ++pair) {
        std::vector<unsigned char> first = word(size, state);
        std::vector<unsigned char> second = word(size, state);
        total += distance(first, second);
    }
    std::printf("%lld edits over 20 pairs of %ld letters\n", total, size);
}
