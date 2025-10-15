use rand::Rng;
use std::time::Instant;

const RADIUS: f64 = 1.0;
const RADIUS_SQ: f64 = RADIUS * RADIUS;

fn generate_coord<T: Rng>(mut rng: T) -> (f64, f64) {
    // Generate an x, y coordinate pair
    let x: f64 = rng.random();
    let y: f64 = rng.random();

    (x, y)
}

fn main() {
    let n_samples: i32 = 10000;
    let mut rng = rand::rng();

    let time_start = Instant::now();
    let mut count = 0;
    for _i in 0..=n_samples {
        let (x, y) = generate_coord(&mut rng);
        let sample_radius_sq = x * x + y * y;
        let in_circle: bool = sample_radius_sq <= RADIUS_SQ;
        if in_circle {
            count += 1
        }
    }
    let time_end = Instant::now();

    let duration = time_end.duration_since(time_start);
    let duration_ms = duration.as_micros();
    
    let pi: f64 = 4.0 * count as f64 / n_samples as f64;

    println!("count: {count}");
    println!("pi: {pi}");
    
    println!("duration: {duration_ms} microseconds");
}
