// Hash map: put a lot of integer keys into the standard library's hash
// map, look each one up, take every other one out, and look them all up
// again. Hashing, probing, and memory for the table.
#include <cstdio>
#include <cstdlib>
#include <unordered_map>
#include <vector>

int main(int argc, char** argv) {
    long long total = argc > 1 ? std::atoll(argv[1]) : 100000;
    std::vector<long long> keys;
    long long state = 42;
    for (long long i = 0; i < total; ++i) {
        state = (state * 1103515245 + 12345) % 2147483648LL;
        keys.push_back(state);
    }
    std::unordered_map<long long, long long> map;
    for (long long i = 0; i < total; ++i) map[keys[i]] = i;
    long long sum = 0;
    for (long long key : keys) sum += map.at(key);
    for (long long i = 0; i < total; i += 2) map.erase(keys[i]);
    long long found = 0;
    for (long long key : keys) {
        if (map.count(key)) ++found;
    }
    std::printf("%lld %lld %lld\n", (long long)map.size(), sum, found);
}
