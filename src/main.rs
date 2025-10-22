use clap::Parser;
use mc_circle::utils::estimate_pi;
use polars::prelude::*;
use std::time::Instant;

#[derive(Parser)]
struct Args {
    // The pattern to look for
    #[arg(short, long)]
    n_samples: i32,
    #[arg(short, long)]
    n_estimates: i32,
}

fn calculate_central_estimate(n_iterations: i32, &n_samples: &i32) -> (f64, f64) {
    let mut estimates = std::vec::Vec::new();
    for _i in 0..n_iterations {
        let pi = estimate_pi(&n_samples);
        estimates.push(pi);
    }
    let s = Series::new("Estimates".into(), &estimates);
    let mean_pi: f64 = s.mean().unwrap();
    let estimate_length = n_iterations as f64;
    let std_err = s.std(1).unwrap() / estimate_length.sqrt();

    (mean_pi, std_err)
}

fn main() {
    let args = Args::parse();
    let n_samples = args.n_samples;
    let n_estimates = args.n_estimates;

    let time_start = Instant::now();
    let (mean_pi, std_err) = calculate_central_estimate(n_estimates, &n_samples);
    let time_end = Instant::now();

    let duration = time_end.duration_since(time_start);
    let duration_ms = duration.as_micros();

    println!("n_samples: {n_samples}");
    println!("mean estimate: {mean_pi} +/- {std_err}");
    println!("duration: {duration_ms} microseconds");
}
