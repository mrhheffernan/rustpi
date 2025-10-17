import argparse
import numpy as np
import time

RADIUS = 1.0
RADIUS_SQ = RADIUS * RADIUS


def parse_args():
    parser = argparse.ArgumentParser()

    parser.add_argument("--n_samples", type=int, help="Number of samples")

    return parser.parse_args()


def generate_coord(rng) -> tuple[float, float]:
    """Generate an x, y coordinate pair"""
    x = rng.random()
    y = rng.random()
    return x, y


def main():
    args = parse_args()
    n_samples = args.n_samples

    rng = np.random.default_rng()

    time_start = time.time()
    count = 0
    x = rng.random(n_samples)
    y = rng.random(n_samples)

    for i in range(n_samples):
        x_sample = x[i]
        y_sample = y[i]
        sample_radius_sq = x_sample * x_sample + y_sample * y_sample
        in_circle = sample_radius_sq <= RADIUS_SQ
        if in_circle:
            count += 1
    time_end = time.time()

    print(f"n_samples: {n_samples}")
    print(f"pi: {4 * count/n_samples}")
    print(f"duration: {(time_end - time_start) * 10 ** 6}  microseconds")


if __name__ == "__main__":
    main()
