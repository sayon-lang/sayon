/*
Spectral norm: the largest singular value of an infinite matrix, worked
out with the power method on an n by n corner of it.
*/
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <vector>

static double entry(long i, long j) {
    return 1.0 / static_cast<double>((i + j) * (i + j + 1) / 2 + i + 1);
}

static void times(const std::vector<double>& v, std::vector<double>& out) {
    long n = static_cast<long>(v.size());
    for (long i = 0; i < n; ++i) {
        double sum = 0;
        for (long j = 0; j < n; ++j) sum += entry(i, j) * v[j];
        out[i] = sum;
    }
}

static void times_transposed(const std::vector<double>& v, std::vector<double>& out) {
    long n = static_cast<long>(v.size());
    for (long i = 0; i < n; ++i) {
        double sum = 0;
        for (long j = 0; j < n; ++j) sum += entry(j, i) * v[j];
        out[i] = sum;
    }
}

static void times_both(const std::vector<double>& v, std::vector<double>& out, std::vector<double>& between) {
    times(v, between);
    times_transposed(between, out);
}

int main(int argc, char** argv) {
    long n = argc > 1 ? std::atol(argv[1]) : 100;
    std::vector<double> u(n, 1.0), v(n, 0.0), between(n, 0.0);
    for (int round = 0; round < 10; ++round) {
        times_both(u, v, between);
        times_both(v, u, between);
    }
    double vbv = 0, vv = 0;
    for (long i = 0; i < n; ++i) {
        vbv += u[i] * v[i];
        vv += v[i] * v[i];
    }
    std::printf("%.9f\n", std::sqrt(vbv / vv));
}
