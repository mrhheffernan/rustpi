use rand::Rng;
use std::time::Instant;

fn generate_coord<T: Rng>(mut rng: T) -> (f64, f64) {
    let x: f64 = rng.random();
    let y: f64 = rng.random();

    (x, y)
}

fn main() {
    let rng = rand::rng();

    let time_start = Instant::now();
    let (x, y) = generate_coord(rng);
    let time_end = Instant::now();

    let duration = time_end.duration_since(time_start);
    let duration_ms = duration.as_micros();

    println!("x: {x}");
    println!("y: {y}");
    println!("duration: {duration_ms} microseconds")
}
