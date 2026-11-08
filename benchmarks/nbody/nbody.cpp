// N-body: the Sun and the four gas giants, moved step by step by each
// other's gravity, with the system's energy before and after.
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <vector>

constexpr double PI = 3.141592653589793;
constexpr double SOLAR_MASS = 4.0 * PI * PI;
constexpr double DAYS_PER_YEAR = 365.24;

struct Body {
    double x, y, z, vx, vy, vz, mass;
};

static Body planet(double x, double y, double z, double vx, double vy, double vz, double mass) {
    return {x, y, z, vx * DAYS_PER_YEAR, vy * DAYS_PER_YEAR, vz * DAYS_PER_YEAR, mass * SOLAR_MASS};
}

static std::vector<Body> solar_system() {
    return {
        {0, 0, 0, 0, 0, 0, SOLAR_MASS},
        planet(4.84143144246472090e+00, -1.16032004402742839e+00, -1.03622044471123109e-01, 1.66007664274403694e-03, 7.69901118419740425e-03, -6.90460016972063023e-05, 9.54791938424326609e-04),
        planet(8.34336671824457987e+00, 4.12479856412430479e+00, -4.03523417114321381e-01, -2.76742510726862411e-03, 4.99852801234917238e-03, 2.30417297573763929e-05, 2.85885980666130812e-04),
        planet(1.28943695621391310e+01, -1.51111514016986312e+01, -2.23307578892655734e-01, 2.96460137564761618e-03, 2.37847173959480950e-03, -2.96589568540237556e-05, 4.36624404335156298e-05),
        planet(1.53796971148509165e+01, -2.59193146099879641e+01, 1.79258772950371181e-01, 2.68067772490389322e-03, 1.62824170038242295e-03, -9.51592254519715870e-05, 5.15138902046611451e-05),
    };
}

static void offset_momentum(std::vector<Body>& bodies) {
    double px = 0, py = 0, pz = 0;
    for (const Body& body : bodies) {
        px += body.vx * body.mass;
        py += body.vy * body.mass;
        pz += body.vz * body.mass;
    }
    bodies[0].vx = -px / SOLAR_MASS;
    bodies[0].vy = -py / SOLAR_MASS;
    bodies[0].vz = -pz / SOLAR_MASS;
}

static double energy(const std::vector<Body>& bodies) {
    double total = 0;
    for (size_t i = 0; i < bodies.size(); ++i) {
        const Body& a = bodies[i];
        total += 0.5 * a.mass * (a.vx * a.vx + a.vy * a.vy + a.vz * a.vz);
        for (size_t j = i + 1; j < bodies.size(); ++j) {
            const Body& b = bodies[j];
            double dx = a.x - b.x, dy = a.y - b.y, dz = a.z - b.z;
            total -= a.mass * b.mass / std::sqrt(dx * dx + dy * dy + dz * dz);
        }
    }
    return total;
}

static void advance(std::vector<Body>& bodies, double dt) {
    for (size_t i = 0; i < bodies.size(); ++i) {
        Body& a = bodies[i];
        for (size_t j = i + 1; j < bodies.size(); ++j) {
            Body& b = bodies[j];
            double dx = a.x - b.x, dy = a.y - b.y, dz = a.z - b.z;
            double distance2 = dx * dx + dy * dy + dz * dz;
            double magnitude = dt / (distance2 * std::sqrt(distance2));
            double mass_a = a.mass * magnitude, mass_b = b.mass * magnitude;
            a.vx -= dx * mass_b;
            a.vy -= dy * mass_b;
            a.vz -= dz * mass_b;
            b.vx += dx * mass_a;
            b.vy += dy * mass_a;
            b.vz += dz * mass_a;
        }
    }
    for (Body& body : bodies) {
        body.x += dt * body.vx;
        body.y += dt * body.vy;
        body.z += dt * body.vz;
    }
}

int main(int argc, char** argv) {
    long steps = argc > 1 ? std::atol(argv[1]) : 1000;
    std::vector<Body> bodies = solar_system();
    offset_momentum(bodies);
    std::printf("%.9f\n", energy(bodies));
    for (long step = 0; step < steps; ++step) advance(bodies, 0.01);
    std::printf("%.9f\n", energy(bodies));
}
