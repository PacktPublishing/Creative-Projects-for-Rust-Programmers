// Declare basic functions for pseudo-random number generators.
use rand::prelude::*;

fn main() {
    // Create a pseudo-Random Number Generator for the current thread
    let mut rng = thread_rng();

    // Print an integer number
    // between 0 (included) and 20 (excluded).
    println!("{}", rng.gen_range(0, 20));

    // Print a floating-point number
    // between 0 (included) and 1 (excluded).
    println!("{}", rng.gen::<f64>());

    // Generate a Boolean.
    println!("{}", if rng.gen() { "Heads" } else { "Tails" });
}
