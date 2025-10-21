# rustpi

The mathematical version of Hello, World: Calculating pi by integrating the unit circle.

The implementation is duplicated in Python for benchmarking, which shows that `cargo run --release` yields a 20x speedup over the matching Python implementation, while without the release optimizations the Rust implementation is actually slower.

Implementing parallelization with Rayon and an anonymous function, but updating the benchmark to use numpy's random generation for a vector doesn't change the speedup ratio: Rust remains 20x faster but coming up on 30x faster at 10M samples.

Further improving the Rayon parallelization and expanding its use reduces the runtime further, but updating the Python benchmark to utilize numpy arrays and vectorization closes the performance gap to only 10x better for rust.