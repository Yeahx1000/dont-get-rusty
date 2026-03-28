//! Collections in Rust
//! This module contains exercises to help you understand:
//! - Vectors
//! - HashMaps
//! - HashSets
//! - Iterators and collection methods
use std::collections::HashMap;

#[allow(unused_imports)]
use crate::utils::{
    create_test_hashmap, create_test_hashset, create_test_strings, create_test_vector,
    print_section_header,
};

// Exercise 1: Vector Operations
// TODO: Write a function that:
// - Takes a vector of integers
// - Returns a new vector containing:
//   - The sum of all numbers
//   - The product of all numbers
//   - The average of all numbers
// Use iterators and collection methods

#[allow(dead_code)]
pub fn excercise1(num: Vec<i32>) -> (i32, i32, f64) {
    let sum: i32 = num.iter().sum();
    let product: i32 = num.iter().product();
    let avg: f64 = sum as f64 / num.len() as f64;

    println!("Sum: {}, Product: {}, Average: {}", sum, product, avg);

    return (sum, product, avg);
}

// Exercise 2: HashMap Basics
// TODO: Write a function that:
// - Creates a HashMap mapping strings to integers
// - Adds some key-value pairs
// - Returns the value for a given key
// Handle the case where the key doesn't exist

#[allow(dead_code)]
pub fn excercise2(key: &str) -> Option<i32> {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);

    if key == "one" {
        println!("Value for 'one': {:?}", map.get("one"));
        return Some(1);
    } else if key == "two" {
        println!("Value for 'two': {:?}", map.get("two"));
        return Some(2);
    } else if key == "three" {
        println!("Value for 'three': {:?}", map.get("three"));
        return Some(3);
    } else {
        println!("Key not found");
        return None;
    }
}

// Exercise 3: HashSet Operations
// TODO: Write a function that:
// - Takes two vectors of integers
// - Returns a HashSet containing:
//   - Numbers that appear in both vectors
//   - Numbers that appear in either vector
// Use HashSet operations (intersection, union)

// Exercise 4: Iterator Chains
// TODO: Write a function that:
// - Takes a vector of strings
// - Returns a new vector containing:
//   - Strings longer than 5 characters
//   - Converted to uppercase
//   - Sorted alphabetically
// Use iterator methods (filter, map, collect)

// Exercise 5: Custom Collection
// TODO: Create a struct named 'NumberSet' that:
// - Stores unique numbers
// - Has methods to:
//   - Add a number
//   - Remove a number
//   - Check if a number exists
//   - Get the sum of all numbers
// Use a HashSet internally

// Bonus Challenge:
// TODO: Write a function that:
// - Takes a string
// - Returns a HashMap containing:
//   - Each character as a key
//   - The count of occurrences as the value
// Use iterators and entry API
