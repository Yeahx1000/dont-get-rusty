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

// Exercise 2: Multiple Return Values
// TODO: Write a function that:
// - Takes a vector of integers
// - Returns a tuple containing:
//   - The sum of all numbers
//   - The average of all numbers
//   - The maximum number

// Exercise 3: Closures
// TODO: Write a function that:
// - Takes a closure as a parameter
// - Applies the closure to a number
// - Returns the result
// Try it with different closures (addition, multiplication, etc.)

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

// Example solutions (commented out):
/*
pub fn run_exercises() {
    // Exercise 1
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    println!("Sum: {}", add(5, 3));

    // Exercise 2
    fn stats(numbers: &[i32]) -> (i32, f64, i32) {
        let sum: i32 = numbers.iter().sum();
        let avg = sum as f64 / numbers.len() as f64;
        let max = *numbers.iter().max().unwrap_or(&0);
        (sum, avg, max)
    }
    let numbers = vec![1, 2, 3, 4, 5];
    let (sum, avg, max) = stats(&numbers);
    println!("Sum: {}, Average: {}, Max: {}", sum, avg, max);

    // Exercise 3
    fn apply_closure<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(x)
    }
    let add_one = |x| x + 1;
    let multiply_by_two = |x| x * 2;
    println!("Add one to 5: {}", apply_closure(add_one, 5));
    println!("Multiply 5 by 2: {}", apply_closure(multiply_by_two, 5));

    // Exercise 4
    fn apply_function(f: fn(i32) -> i32, start: i32, end: i32) -> Vec<i32> {
        (start..=end).map(f).collect()
    }
    fn square(x: i32) -> i32 {
        x * x
    }
    let squares = apply_function(square, 1, 5);
    println!("Squares: {:?}", squares);

    // Exercise 5
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn perimeter(&self) -> u32 {
            2 * (self.width + self.height)
        }
        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }
    let rect = Rectangle::new(5, 5);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square: {}", rect.is_square());
}
*/ 