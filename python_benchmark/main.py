import argparse
import time

import numpy as np

RADIUS = 1.0
RADIUS_SQ = RADIUS * RADIUS


def parse_args():
    parser = argparse.ArgumentParser()

    parser.add_argument(
        "--n_samples", type=int, help="Number of samples", required=True
    )

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
    coords = rng.random((n_samples, 2))
    radii = coords[:, 0] ** 2 + coords[:, 1] ** 2
    in_circle = radii <= RADIUS_SQ
    count = sum(in_circle)

    time_end = time.time()

    print(f"n_samples: {n_samples}")
    print(f"pi: {4 * count / n_samples}")
    print(f"duration: {(time_end - time_start) * 10**6}  microseconds")


if __name__ == "__main__":
    main()
