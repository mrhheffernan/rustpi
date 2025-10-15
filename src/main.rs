use rand::Rng;

fn main() {
    let mut rng = rand::rng();

    let x: f64 = rng.random();
    let y: f64 = rng.random();

    println!("x: {x}");
    println!("y: {y}");
}
