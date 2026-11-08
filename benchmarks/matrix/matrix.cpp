// Matrix: multiply two square matrices of floating point numbers, each
// kept row by row in one array. Three nested loops over arrays, the kind
// of work compilers like to turn into vector instructions.
#include <cstdio>
#include <cstdlib>
#include <vector>

static void multiply(const std::vector<double>& a, const std::vector<double>& b, std::vector<double>& c, long long n) {
    for (long long i = 0; i < n; ++i) {
        for (long long k = 0; k < n; ++k) {
            double factor = a[i * n + k];
            for (long long j = 0; j < n; ++j) c[i * n + j] += factor * b[k * n + j];
        }
    }
}

int main(int argc, char** argv) {
    long long n = argc > 1 ? std::atoll(argv[1]) : 200;
    std::vector<double> a, b, c;
    for (long long i = 0; i < n; ++i) {
        for (long long j = 0; j < n; ++j) {
            a.push_back(double((i + j) % 10) / 10.0);
            b.push_back(double((i * j) % 7) / 7.0);
            c.push_back(0.0);
        }
    }
    multiply(a, b, c, n);
    double total = 0.0;
    for (double value : c) total += value;
    std::printf("%.6f %.6f\n", total, c[(n / 2) * n + n / 3]);
}
