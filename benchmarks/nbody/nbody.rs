// N-body: the Sun and the four gas giants, moved step by step by each
// other's gravity, with the system's energy before and after.

const PI: f64 = 3.141592653589793;
const SOLAR_MASS: f64 = 4.0 * PI * PI;
const DAYS_PER_YEAR: f64 = 365.24;

#[derive(Clone, Copy)]
struct Body {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    mass: f64,
}

fn planet(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64, mass: f64) -> Body {
    Body { x, y, z, vx: vx * DAYS_PER_YEAR, vy: vy * DAYS_PER_YEAR, vz: vz * DAYS_PER_YEAR, mass: mass * SOLAR_MASS }
}

fn solar_system() -> Vec<Body> {
    vec![
        Body { x: 0.0, y: 0.0, z: 0.0, vx: 0.0, vy: 0.0, vz: 0.0, mass: SOLAR_MASS },
        planet(4.84143144246472090e+00, -1.16032004402742839e+00, -1.03622044471123109e-01, 1.66007664274403694e-03, 7.69901118419740425e-03, -6.90460016972063023e-05, 9.54791938424326609e-04),
        planet(8.34336671824457987e+00, 4.12479856412430479e+00, -4.03523417114321381e-01, -2.76742510726862411e-03, 4.99852801234917238e-03, 2.30417297573763929e-05, 2.85885980666130812e-04),
        planet(1.28943695621391310e+01, -1.51111514016986312e+01, -2.23307578892655734e-01, 2.96460137564761618e-03, 2.37847173959480950e-03, -2.96589568540237556e-05, 4.36624404335156298e-05),
        planet(1.53796971148509165e+01, -2.59193146099879641e+01, 1.79258772950371181e-01, 2.68067772490389322e-03, 1.62824170038242295e-03, -9.51592254519715870e-05, 5.15138902046611451e-05),
    ]
}

fn offset_momentum(bodies: &mut [Body]) {
    let (mut px, mut py, mut pz) = (0.0, 0.0, 0.0);
    for body in bodies.iter() {
        px += body.vx * body.mass;
        py += body.vy * body.mass;
        pz += body.vz * body.mass;
    }
    bodies[0].vx = -px / SOLAR_MASS;
    bodies[0].vy = -py / SOLAR_MASS;
    bodies[0].vz = -pz / SOLAR_MASS;
}

fn energy(bodies: &[Body]) -> f64 {
    let mut total = 0.0;
    for (i, a) in bodies.iter().enumerate() {
        total += 0.5 * a.mass * (a.vx * a.vx + a.vy * a.vy + a.vz * a.vz);
        for b in &bodies[i + 1..] {
            let (dx, dy, dz) = (a.x - b.x, a.y - b.y, a.z - b.z);
            total -= a.mass * b.mass / (dx * dx + dy * dy + dz * dz).sqrt();
        }
    }
    total
}

fn advance(bodies: &mut [Body], dt: f64) {
    for i in 0..bodies.len() {
        let (left, right) = bodies.split_at_mut(i + 1);
        let a = &mut left[i];
        for b in right.iter_mut() {
            let (dx, dy, dz) = (a.x - b.x, a.y - b.y, a.z - b.z);
            let distance2 = dx * dx + dy * dy + dz * dz;
            let magnitude = dt / (distance2 * distance2.sqrt());
            let (mass_a, mass_b) = (a.mass * magnitude, b.mass * magnitude);
            a.vx -= dx * mass_b;
            a.vy -= dy * mass_b;
            a.vz -= dz * mass_b;
            b.vx += dx * mass_a;
            b.vy += dy * mass_a;
            b.vz += dz * mass_a;
        }
    }
    for body in bodies.iter_mut() {
        body.x += dt * body.vx;
        body.y += dt * body.vy;
        body.z += dt * body.vz;
    }
}

fn main() {
    let steps: u64 = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(1000);
    let mut bodies = solar_system();
    offset_momentum(&mut bodies);
    println!("{:.9}", energy(&bodies));
    for _ in 0..steps {
        advance(&mut bodies, 0.01);
    }
    println!("{:.9}", energy(&bodies));
}
