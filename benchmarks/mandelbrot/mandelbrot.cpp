// Mandelbrot: count the points of a size by size grid over the plane that
// stay in the Mandelbrot set for 50 steps.
#include <cstdio>
#include <cstdlib>

static bool inside(double cr, double ci) {
    double zr = 0, zi = 0;
    for (int step = 0; step < 50; ++step) {
        double next_r = zr * zr - zi * zi + cr;
        zi = 2.0 * zr * zi + ci;
        zr = next_r;
        if (zr * zr + zi * zi > 4.0) return false;
    }
    return true;
}

int main(int argc, char** argv) {
    long size = argc > 1 ? std::atol(argv[1]) : 200;
    long count = 0;
    for (long y = 0; y < size; ++y) {
        double ci = 2.0 * static_cast<double>(y) / static_cast<double>(size) - 1.0;
        for (long x = 0; x < size; ++x) {
            double cr = 2.0 * static_cast<double>(x) / static_cast<double>(size) - 1.5;
            if (inside(cr, ci)) ++count;
        }
    }
    std::printf("%ld of %ld points are in the set\n", count, size * size);
}
