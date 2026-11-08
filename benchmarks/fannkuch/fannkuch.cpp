// Fannkuch-redux: flip a stack of pancakes, numbered 1 to n, for every
// order they can start in, and count the most flips any order needs.
#include <cstdio>
#include <cstdlib>
#include <utility>
#include <vector>

int main(int argc, char** argv) {
    int n = argc > 1 ? std::atoi(argv[1]) : 10;
    std::vector<int> perm(n), perm1(n), count(n);
    for (int i = 0; i < n; ++i) perm1[i] = i;
    long long checksum = 0;
    int most = 0, index = 0, r = n;
    while (true) {
        while (r != 1) {
            count[r - 1] = r;
            --r;
        }
        perm = perm1;
        int flips = 0;
        for (int k = perm[0]; k != 0; k = perm[0]) {
            for (int low = 0, high = k; low < high; ++low, --high) std::swap(perm[low], perm[high]);
            ++flips;
        }
        if (flips > most) most = flips;
        checksum += index % 2 == 0 ? flips : -flips;
        while (true) {
            if (r == n) {
                std::printf("%lld\nPfannkuchen(%d) = %d\n", checksum, n, most);
                return 0;
            }
            int first = perm1[0];
            for (int i = 0; i < r; ++i) perm1[i] = perm1[i + 1];
            perm1[r] = first;
            if (--count[r] > 0) break;
            ++r;
        }
        ++index;
    }
}
