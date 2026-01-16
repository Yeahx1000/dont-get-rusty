//! Variables and Basic Types in Rust
//! This module contains exercises to help you understand:
//! - Variable declarations and mutability
//! - Basic types (integers, floats, booleans, characters)
//! - Type inference
//! - Constants
//! - Shadowing

// Exercise 1: Variable Declaration and Mutability
// TODO: Declare a mutable variable named 'age' with the value 25
// TODO: Print the age variable
// TODO: Change the age to 26 and print it again

#[allow(dead_code)]
pub fn exercise1() {
    let mut age: i32 = 25;
    println!("Age: {}", age);

    age = 26;
    println!("New Age: {}", age);
}

// Exercise 2: Type Inference
// TODO: Declare variables with different types and let Rust infer their types
// - An integer (i32)
// - A floating-point number (f64)
// - A boolean
// - A character
// Print each variable and its type

pub fn excercise2() {
    let integer = 42;
    let float = 3.14;
    let bool = true;
    let char = 'A';

    println!("Interger: {}", integer);
    println!("Float: {}", float);
    println!("Bollean: {}", bool);
    println!("Character: {}", char);
}

// Exercise 3: Constants
// TODO: Declare a constant named MAX_POINTS with the value 100_000
// TODO: Print the constant

pub fn excercise3() {
    const MAX_POINTS: i32 = 10_000;
    println!("Maximum amount of points you can have is {}", MAX_POINTS);
}

// Exercise 4: Shadowing
// TODO: Declare a variable named 'x' with value 5
// TODO: Shadow it with a new value that is the original value plus 1
// TODO: Shadow it again with a new value that is the previous value multiplied by 2
// Print each value of x

pub fn excercise4() {
    let x: i32 = 5;
    let x2: i32 = x + 1;
    let x3: i32 = x2 * 2;

    println!("x: {}, x2: {}, x3: {}", x, x2, x3);
}

// Exercise 5: Type Annotations
// TODO: Declare variables with explicit type annotations
// - A 32-bit integer
// - A 64-bit float
// - A boolean
// Print each variable

pub fn excercise5() {
    const INTEGER: i32 = 42;
    const FLOAT: f64 = 3.14;
    const BOOLEAN: bool = true;

    println!("integer: {}", INTEGER);
    println!("float: {}", FLOAT);
    println!("boolean: {}", BOOLEAN);
}

// Bonus Challenge:
// TODO: Try to create a variable with a value that's too large for its type
// What happens when you try to compile?

pub fn excercise_bonus() {
    #[allow(overflowing_literals)]
    let x: i32 = 123_456_789_123_456_789;
    println!("x: {}", x);
}
