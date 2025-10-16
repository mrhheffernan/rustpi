use rand::Rng;

const RADIUS: f64 = 1.0;
const RADIUS_SQ: f64 = RADIUS * RADIUS;

pub mod utils {
    fn generate_coord<T: super::Rng>(mut rng: T) -> (f64, f64) {
        // Generate an x, y coordinate pair
        let x: f64 = rng.random();
        let y: f64 = rng.random();

        (x, y)
    }

    pub fn estimate_pi<T: super::Rng>(&n_samples: &i32, mut rng: T) -> f64 {
        let mut count = 0;
        for _i in 0..=n_samples {
            let (x, y) = generate_coord(&mut rng);
            let sample_radius_sq = x * x + y * y;
            let in_circle: bool = sample_radius_sq <= super::RADIUS_SQ;
            if in_circle {
                count += 1
            }
        }

        let pi: f64 = 4.0 * count as f64 / n_samples as f64;
        pi
    }
}
