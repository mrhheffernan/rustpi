const RADIUS: f64 = 1.0;
const RADIUS_SQ: f64 = RADIUS * RADIUS;

pub mod utils {
    use rand::Rng;

    fn generate_coords(n_samples: i32) -> Vec<(f64, f64)> {
        // the || is a way of getting an anonymous functino
        let sampler = || {
            let mut rng = rand::rng();
            rng.random_range(0.0..1.0)
        };

        let mut pairs: Vec<(f64, f64)> = Vec::new();
        rayon::ThreadPoolBuilder::new()
            .build()
            .unwrap()
            .install(|| {
                for _ in 0..n_samples {
                    let x = sampler();
                    let y = sampler();
                    pairs.push((x, y))
                }
            });

        pairs
    }

    pub fn estimate_pi(&n_samples: &i32) -> f64 {
        let mut count = 0;
        let coords = generate_coords(n_samples);

        for i in 0..=n_samples - 1 {
            let (x_sample, y_sample) = coords[i as usize];

            let sample_radius_sq = x_sample * x_sample + y_sample * y_sample;
            let in_circle: bool = sample_radius_sq <= super::RADIUS_SQ;

            if in_circle {
                count += 1
            }
        }

        let pi: f64 = 4.0 * count as f64 / n_samples as f64;
        pi
    }
}
