# rustpi

The mathematical version of Hello, World: Calculating pi by integrating the unit circle.

The implementation is duplicated in Python for benchmarking, which shows that `cargo run --release` yields a 20x speedup over the matching Python implementation, while without the release optimizations the Rust implementation is actually slower.

Implementing parallelization with Rayon and an anonymous function, but updating the benchmark to use numpy's random generation for a vector doesn't change the speedup ratio: Rust remains 20x faster.