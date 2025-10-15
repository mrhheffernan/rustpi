# rustpi

The mathematical version of Hello, World: Calculating pi by integrating the unit circle.

The implementation is duplicated in Python for benchmarking, which shows that `cargo run --release` yields a 20x speedup over the matching Python implementation, while without the release optimizations the Rust implementation is actually slower.