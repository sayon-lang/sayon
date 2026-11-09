/*
Sieve: count the primes below a limit with the sieve of Eratosthenes,
one byte for each number. A long walk through memory in simple steps.
*/
#include <cstdio>
#include <cstdlib>
#include <vector>

int main(int argc, char** argv) {
    long long limit = argc > 1 ? std::atoll(argv[1]) : 1000000;
    /*
    A byte for each number, as in the other versions; std::vector<bool>
    would pack eight numbers to a byte.
    */
    std::vector<unsigned char> composite(limit, 0);
    long long count = 0;
    long long last = 0;
    for (long long n = 2; n < limit; ++n) {
        if (composite[n]) continue;
        ++count;
        last = n;
        for (long long multiple = n * n; multiple < limit; multiple += n) composite[multiple] = 1;
    }
    std::printf("%lld %lld\n", count, last);
}
