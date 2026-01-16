//! Error Handling in Rust
//! This module contains exercises to help you understand:
//! - Result type
//! - Option type
//! - Custom error types
//! - Error propagation
//! - Error handling patterns

#[allow(unused_imports)]
use crate::utils::{
    create_test_errors, create_test_numbers, create_test_options, create_test_string,
    print_section_header,
};

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

