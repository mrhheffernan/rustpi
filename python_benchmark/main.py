import numpy as np
import time

RADIUS = 1.0
RADIUS_SQ = RADIUS * RADIUS


def generate_coord(rng) -> tuple[float, float]:
    """Generate an x, y coordinate pair"""
    x = rng.random()
    y = rng.random()
    return x, y


def main():
    n_samples = 100000
    rng = np.random.default_rng()

    time_start = time.time()
    count = 0
    for i in range(n_samples):
        x, y = generate_coord(rng)
        sample_radius_sq = x * x + y * y
        in_circle = sample_radius_sq <= RADIUS_SQ
        if in_circle:
            count += 1
    time_end = time.time()

    print(f"pi: {4 * count/n_samples}")
    print(f"duration: {(time_end - time_start) * 10 ** 6}  microseconds")


if __name__ == "__main__":
    main()
