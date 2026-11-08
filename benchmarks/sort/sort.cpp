// Sort: fill an array with pseudo-random integers and sort it with the
// standard library's stable sort, as Sayon's sort is stable.
#include <algorithm>
#include <cstdio>
#include <cstdlib>
#include <vector>

int main(int argc, char** argv) {
    long total = argc > 1 ? std::atol(argv[1]) : 100000;
    std::vector<long long> numbers;
    long long state = 42;
    for (long i = 0; i < total; ++i) {
        state = (state * 1103515245 + 12345) % 2147483648LL;
        numbers.push_back(state);
    }
    std::stable_sort(numbers.begin(), numbers.end());
    long long checksum = 0;
    for (long position = 0; position < total; position += 1000) checksum += numbers[position];
    std::printf("%lld %lld %lld %lld\n", numbers[0], numbers[total / 2], numbers[total - 1], checksum);
}
