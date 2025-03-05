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

fn exercise1() {
    let mut age = 25;
    println!("Ager {}", age);

    age = 26;
    println!("New Age {}", age);
}



// Exercise 2: Type Inference
// TODO: Declare variables with different types and let Rust infer their types
// - An integer (i32)
// - A floating-point number (f64)
// - A boolean
// - A character
// Print each variable and its type

// Exercise 3: Constants
// TODO: Declare a constant named MAX_POINTS with the value 100_000
// TODO: Print the constant

// Exercise 4: Shadowing
// TODO: Declare a variable named 'x' with value 5
// TODO: Shadow it with a new value that is the original value plus 1
// TODO: Shadow it again with a new value that is the previous value multiplied by 2
// Print each value of x

// Exercise 5: Type Annotations
// TODO: Declare variables with explicit type annotations
// - A 32-bit integer
// - A 64-bit float
// - A boolean
// Print each variable

// Bonus Challenge:
// TODO: Try to create a variable with a value that's too large for its type
// What happens when you try to compile?

// Example solutions (commented out):
/*
pub fn run_exercises() {
    // Exercise 1
    let mut age = 25;
    println!("Age: {}", age);
    age = 26;
    println!("New age: {}", age);

    // Exercise 2
    let integer = 42;
    let float = 3.14;
    let boolean = true;
    let character = 'A';
    println!("Integer: {} (type: {})", integer, std::mem::size_of_val(&integer));
    println!("Float: {} (type: {})", float, std::mem::size_of_val(&float));
    println!("Boolean: {} (type: {})", boolean, std::mem::size_of_val(&boolean));
    println!("Character: {} (type: {})", character, std::mem::size_of_val(&character));

    // Exercise 3
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);

    // Exercise 4
    let x = 5;
    println!("First x: {}", x);
    let x = x + 1;
    println!("Second x: {}", x);
    let x = x * 2;
    println!("Third x: {}", x);

    // Exercise 5
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    println!("Annotated integer: {}", integer);
    println!("Annotated float: {}", float);
    println!("Annotated boolean: {}", boolean);
}
*/ 