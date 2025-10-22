use clap::Parser;
use mc_circle::utils::estimate_pi;
use polars::prelude::*;
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
    const N_ESTIMATES: i32 = 10;

    let time_start = Instant::now();

    let mut estimates = std::vec::Vec::new();
    for _i in 0..N_ESTIMATES {
        let pi = estimate_pi(&n_samples);
        estimates.push(pi);
    }

    let s = Series::new("Estimates".into(), &estimates);
    let mean_pi: f64 = s.mean().unwrap();
    let estimate_length = N_ESTIMATES as f64;
    let std_err = s.std(1).unwrap() / estimate_length.sqrt();

    let time_end = Instant::now();

    let duration = time_end.duration_since(time_start);
    let duration_ms = duration.as_micros();

    println!("n_samples: {n_samples}");
    println!("mean estimate: {mean_pi} +/- {std_err}");
    println!("duration: {duration_ms} microseconds");
}
