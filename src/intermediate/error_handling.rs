//! Error Handling in Rust
//! This module contains exercises to help you understand:
//! - Result type
//! - Option type
//! - Custom error types
//! - Error propagation
//! - Error handling patterns

use crate::utils::{print_section_header, create_test_string, create_test_numbers, create_test_errors, create_test_options};

// Exercise 1: Basic Result
// TODO: Write a function that:
// - Takes a string
// - Returns Result<String, String>
// - Returns Ok with uppercase string if length > 5
// - Returns Err with error message if length <= 5

// Exercise 2: Option Type
// TODO: Write a function that:
// - Takes a vector of integers
// - Returns Option<i32>
// - Returns Some with first even number
// - Returns None if no even numbers found

// Exercise 3: Custom Error Type
// TODO: Create a custom error type for a calculator that:
// - Handles division by zero
// - Handles invalid operations
// - Implements std::error::Error
// - Has descriptive error messages

// Exercise 4: Error Propagation
// TODO: Write a function that:
// - Reads a file
// - Parses its contents as JSON
// - Returns Result with parsed data
// - Uses ? operator for error propagation
// Handle multiple error types

// Exercise 5: Error Handling Patterns
// TODO: Write a function that:
// - Takes a string
// - Attempts to parse it as a number
// - Returns Result with:
//   - Parsed number if successful
//   - Custom error if parsing fails
// Use map_err and other combinators

// Bonus Challenge:
// TODO: Write a function that:
// - Takes a URL string
// - Makes an HTTP request
// - Returns Result with response
// Handle network errors, invalid URLs, etc.

// Example solutions (commented out):
/*
pub fn run_exercises() {
    use std::error::Error;
    use std::fmt;
    use serde_json::Value;
    use std::fs;

    print_section_header("Error Handling Exercises");

    // Exercise 1
    fn process_string(s: String) -> Result<String, String> {
        if s.len() > 5 {
            Ok(s.to_uppercase())
        } else {
            Err("String too short".to_string())
        }
    }
    let s = create_test_string();
    match process_string(s) {
        Ok(s) => println!("Processed: {}", s),
        Err(e) => println!("Error: {}", e),
    }
    print_test_result("String processing", true);

    // Exercise 2
    fn find_first_even(v: &[i32]) -> Option<i32> {
        v.iter().find(|&&x| x % 2 == 0).copied()
    }
    let v = create_test_numbers();
    match find_first_even(&v) {
        Some(n) => println!("First even: {}", n),
        None => println!("No even numbers found"),
    }
    print_test_result("Find first even", true);

    // Exercise 3
    #[derive(Debug)]
    enum CalculatorError {
        DivisionByZero,
        InvalidOperation(String),
    }
    impl fmt::Display for CalculatorError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                CalculatorError::DivisionByZero => write!(f, "Division by zero"),
                CalculatorError::InvalidOperation(op) => write!(f, "Invalid operation: {}", op),
            }
        }
    }
    impl Error for CalculatorError {}
    print_test_result("Custom error type", true);

    // Exercise 4
    fn read_json_file(path: &str) -> Result<Value, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let value: Value = serde_json::from_str(&contents)?;
        Ok(value)
    }
    print_test_result("Error propagation", true);

    // Exercise 5
    fn parse_number(s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|e| format!("Failed to parse number: {}", e))
    }
    match parse_number("42") {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Error: {}", e),
    }
    print_test_result("Number parsing", true);

    // Bonus: Process test errors
    let errors = create_test_errors();
    for result in errors {
        match result {
            Ok(n) => println!("Success: {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    // Process test options
    let options = create_test_options();
    for opt in options {
        match opt {
            Some(n) => println!("Some: {}", n),
            None => println!("None"),
        }
    }
}
*/ 