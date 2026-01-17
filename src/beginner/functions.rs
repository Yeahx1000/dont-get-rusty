//! Functions and Closures in Rust
//! This module contains exercises to help you understand:
//! - Function declarations and parameters
//! - Return values
//! - Closures
//! - Function pointers
//! - Methods

// Exercise 1: Basic Functions
// TODO: Write a function that:
// - Takes two integers as parameters
// - Returns their sum
// - Has explicit type annotations

#[allow(dead_code)]
pub fn excercise1(a: i32, b: i32) -> i32 {
    let result: i32 = a + b;
    println!("the sum of {} and {} is {}", a, b, result);
    result
}

// Exercise 2: Multiple Return Values
// TODO: Write a function that:
// - Takes a vector of integers
// - Returns a tuple containing:
//   - The sum of all numbers
//   - The average of all numbers
//   - The maximum number

#[allow(dead_code)]
pub fn excercise2(numbers: Vec<i32>) -> (i32, f64, i32) {
    let sum: i32 = numbers.iter().sum();
    let avg: f64 = sum as f64 / numbers.len() as f64;
    let max: i32 = *numbers.iter().max().unwrap_or(&0);
    println!("Sum: {}, Average: {}, Max: {}", sum, avg, max);
    (sum, avg, max)
}

// Exercise 3: Closures
// TODO: Write a function that:
// - Takes a closure as a parameter
// - Applies the closure to a number
// - Returns the result
// Try it with different closures (addition, multiplication, etc.)

#[allow(dead_code)]
pub fn excercise3() {}

// Exercise 4: Function Pointers
// TODO: Write a function that:
// - Takes a function pointer as a parameter
// - Applies the function to a range of numbers
// - Returns a vector of results

// Exercise 5: Methods
// TODO: Create a struct named Rectangle with width and height
// TODO: Implement methods for Rectangle:
// - new() - creates a new rectangle
// - area() - calculates the area
// - perimeter() - calculates the perimeter
// - is_square() - checks if it's a square

// Bonus Challenge:
// TODO: Write a function that takes a closure and returns a new closure
// that applies the original closure twice
