// Declare our modules
mod advanced;
mod beginner;
mod bonus;
mod intermediate;
mod utils;

#[allow(unused_imports)]
use beginner::control_flow;
#[allow(unused_imports)]
use beginner::functions;
#[allow(unused_imports)]
use beginner::variables;

fn main() {
    println!("Welcome to Don't Get Rusty!");
    println!("This project will help you learn Rust from beginner to advanced concepts.");
    println!("\nTo get started:");
    println!("1. Navigate to src/beginner/variables.rs");
    println!("2. Read the comments and complete the exercises");
    println!("3. Run 'cargo run' to test your solutions");
    println!("\nGood luck on your Rust journey! 🦀");
    println!("--------------------------------");

    // Example beginner exercises (uncomment to run):
    // variables::exercise1();

    // Example advanced exercises using dependencies:
    advanced::concurrency::parallel_processing_example();

    println!("\nAll examples completed! Check out the exercises in each module.");
}
