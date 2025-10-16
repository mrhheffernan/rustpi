use clap::Parser;
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

fn estimate_pi<T: Rng>(&n_samples: &i32, mut rng: T) -> f64 {
    let mut count = 0;
    for _i in 0..=n_samples {
        let (x, y) = generate_coord(&mut rng);
        let sample_radius_sq = x * x + y * y;
        let in_circle: bool = sample_radius_sq <= RADIUS_SQ;
        if in_circle {
            count += 1
        }
    }

    let pi: f64 = 4.0 * count as f64 / n_samples as f64;

    pi
}

#[derive(Parser)]
struct Args {
    // The pattern to look for
    #[arg(short, long)]
    n_samples: i32,
}

fn main() {
    let args = Args::parse();
    let n_samples = args.n_samples;
    let mut rng = rand::rng();

    let time_start = Instant::now();
    let pi = estimate_pi(&n_samples, &mut rng);
    let time_end = Instant::now();

    let duration = time_end.duration_since(time_start);
    let duration_ms = duration.as_micros();
    
    println!("n_samples: {n_samples}");
    println!("pi: {pi}");    
    println!("duration: {duration_ms} microseconds");
}
