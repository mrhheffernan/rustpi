use clap::Parser;
use mc_circle::utils::calculate_central_estimate;
use std::time::Instant;

#[derive(Parser)]
struct Args {
    // The pattern to look for
    #[arg(short, long)]
    n_samples: i32,
    #[arg(short, long)]
    n_estimates: i32,
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
