use clap::Parser;
use mc_circle::utils::estimate_pi;
use std::time::Instant;

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
