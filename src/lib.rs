const RADIUS: f64 = 1.0;
const RADIUS_SQ: f64 = RADIUS * RADIUS;

pub mod utils {
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

    pub fn estimate_pi(&n_samples: &i32) -> f64 {
        let coords = generate_coords(n_samples);

        let inside: Vec<bool> = coords
            .into_par_iter()
            .map(|(x_s, y_s)| (x_s * x_s + y_s * y_s) <= super::RADIUS_SQ)
            .collect();
        let count_map: f64 = inside.into_iter().filter(|&b| b).count() as f64;

        let pi: f64 = 4.0 * count_map / n_samples as f64;
        pi
    }
}
