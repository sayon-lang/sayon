/*
Fib: the Fibonacci numbers worked out the slow way, by a function that
calls itself twice for each number. Nothing but calls, comparisons, and
additions, so it shows what a function call costs.
*/
#include <cstdio>
#include <cstdlib>

static long long fib(long long n) {
    if (n < 2) return n;
    return fib(n - 1) + fib(n - 2);
}

int main(int argc, char** argv) {
    long long n = argc > 1 ? std::atoll(argv[1]) : 30;
    std::printf("%lld\n", fib(n));
}
