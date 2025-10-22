const RADIUS: f64 = 1.0;
const RADIUS_SQ: f64 = RADIUS * RADIUS;

pub mod utils {
    use polars::prelude::*;
    use rand::Rng;
    use rayon::prelude::*;
    fn generate_coords(n_samples: i32) -> Vec<(f64, f64)> {
        (0..n_samples)
            .into_par_iter()
            .map_init(rand::rng, |rng, _| {
                (rng.random_range(0.0..1.0), rng.random_range(0.0..1.0))
            })
            .collect()
    }

    fn estimate_pi(&n_samples: &i32) -> f64 {
        let coords = generate_coords(n_samples);

        let inside: Vec<bool> = coords
            .into_par_iter()
            .map(|(x_s, y_s)| (x_s * x_s + y_s * y_s) <= super::RADIUS_SQ)
            .collect();
        let count_map: f64 = inside.into_iter().filter(|&b| b).count() as f64;

        let pi: f64 = 4.0 * count_map / n_samples as f64;
        pi
    }

    pub fn calculate_central_estimate(n_iterations: i32, &n_samples: &i32) -> (f64, f64) {
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
}
