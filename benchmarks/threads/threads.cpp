// Threads: count the steps of the Collatz sequence from every number below
// a limit, the numbers shared out among four threads. Plain integer work
// spread over the processor's cores, so time on the clock falls as the
// cores share it.
#include <cstdio>
#include <cstdlib>
#include <thread>
#include <vector>

struct Share {
    long long total = 0;
    long long longest = 0;
    long long start = 0;
};

static long long steps(long long n) {
    long long count = 0;
    while (n != 1) {
        n = n % 2 == 0 ? n / 2 : 3 * n + 1;
        ++count;
    }
    return count;
}

static Share work(long long low, long long high) {
    Share share;
    for (long long n = low; n < high; ++n) {
        long long count = steps(n);
        share.total += count;
        if (count > share.longest) {
            share.longest = count;
            share.start = n;
        }
    }
    return share;
}

int main(int argc, char** argv) {
    long long limit = argc > 1 ? std::atoll(argv[1]) : 100000;
    const long long parts = 4;
    std::vector<Share> shares(parts);
    std::vector<std::thread> workers;
    for (long long part = 0; part < parts; ++part) {
        long long low = 1 + (limit - 1) * part / parts;
        long long high = 1 + (limit - 1) * (part + 1) / parts;
        workers.emplace_back([&shares, part, low, high] { shares[part] = work(low, high); });
    }
    for (std::thread& worker : workers) worker.join();
    Share all;
    for (const Share& share : shares) {
        all.total += share.total;
        if (share.longest > all.longest) {
            all.longest = share.longest;
            all.start = share.start;
        }
    }
    std::printf("%lld %lld %lld\n", all.total, all.start, all.longest);
}
