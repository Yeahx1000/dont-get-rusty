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
pub fn excercise3(x: i32) -> i32 {
    let add_one = |x| x + 1;
    let multiply_by_two = |x| x * 2;
    println!(
        "the result of adding by 1 and multiplying by two is = {}",
        multiply_by_two(add_one(x))
    );
    multiply_by_two(add_one(x))
}

// Exercise 4: Function Pointers
// TODO: Write a function that:
// - Takes a function pointer as a parameter
// - Applies the function to a range of numbers
// - Returns a vector of results

#[allow(dead_code)]
pub fn pointer_func(f: fn(i32) -> i32, start: i32, end: i32) -> Vec<i32> {
    (start..=end).map(f).collect()
}

#[allow(dead_code)]
pub fn excercise4(a: i32, b: i32) -> () {
    fn square(x: i32) -> i32 {
        x * x
    }

    let squared: Vec<i32> = pointer_func(square, a, b);
    println!(
        "the range of the numbers {} and {} squared is {:?}",
        a, b, squared
    )
}

// Exercise 5: Methods
// TODO: Create a struct named Rectangle with width and height
// TODO: Implement methods for Rectangle:
// - new() - creates a new rectangle
// - area() - calculates the area
// - perimeter() - calculates the perimeter
// - is_square() - checks if it's a square

#[allow(dead_code)]
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

pub fn excercise5(a: u32, b: u32) {
    let new_rectangle = Rectangle::new(a, b);

    println!("The area of {} and {} is {}", a, b, new_rectangle.area());
    println!(
        "The perimeter of {} and {} is {}",
        a,
        b,
        new_rectangle.perimeter()
    );
    println!(
        "Are the sides of width {} and height {} squared? {}",
        a,
        b,
        new_rectangle.is_square()
    )
}

// Bonus Challenge:
// TODO: Write a function that takes a closure and returns a new closure
// that applies the original closure twice
