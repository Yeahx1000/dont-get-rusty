//! Collections in Rust
//! This module contains exercises to help you understand:
//! - Vectors
//! - HashMaps
//! - HashSets
//! - Iterators and collection methods

use crate::utils::{print_section_header, create_test_vector, create_test_hashmap, create_test_hashset, create_test_strings};

// Exercise 1: Vector Operations
// TODO: Write a function that:
// - Takes a vector of integers
// - Returns a new vector containing:
//   - The sum of all numbers
//   - The product of all numbers
//   - The average of all numbers
// Use iterators and collection methods

// Exercise 2: HashMap Basics
// TODO: Write a function that:
// - Creates a HashMap mapping strings to integers
// - Adds some key-value pairs
// - Returns the value for a given key
// Handle the case where the key doesn't exist

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

// Example solutions (commented out):
/*
pub fn run_exercises() {
    use std::collections::{HashMap, HashSet};

    print_section_header("Collections Exercises");

    // Exercise 1
    fn vector_stats(v: &[i32]) -> Vec<f64> {
        let sum: i32 = v.iter().sum();
        let product: i32 = v.iter().product();
        let avg = sum as f64 / v.len() as f64;
        vec![sum as f64, product as f64, avg]
    }
    let v = create_test_vector();
    let stats = vector_stats(&v);
    println!("Vector stats: {:?}", stats);
    print_test_result("Vector stats", true);

    // Exercise 2
    fn get_value(map: &HashMap<String, i32>, key: &str) -> Option<&i32> {
        map.get(key)
    }
    let map = create_test_hashmap();
    println!("Value for 'one': {:?}", get_value(&map, "one"));
    println!("Value for 'three': {:?}", get_value(&map, "three"));
    print_test_result("HashMap get", true);

    // Exercise 3
    fn set_operations(v1: &[i32], v2: &[i32]) -> (HashSet<i32>, HashSet<i32>) {
        let set1: HashSet<_> = v1.iter().cloned().collect();
        let set2: HashSet<_> = v2.iter().cloned().collect();
        (set1.intersection(&set2).cloned().collect(),
         set1.union(&set2).cloned().collect())
    }
    let v1 = create_test_vector();
    let v2 = vec![3, 4, 5, 6, 7];
    let (intersection, union) = set_operations(&v1, &v2);
    println!("Intersection: {:?}", intersection);
    println!("Union: {:?}", union);
    print_test_result("Set operations", true);

    // Exercise 4
    fn process_strings(strings: Vec<String>) -> Vec<String> {
        strings.into_iter()
            .filter(|s| s.len() > 5)
            .map(|s| s.to_uppercase())
            .collect::<Vec<_>>()
    }
    let strings = create_test_strings();
    println!("Processed strings: {:?}", process_strings(strings));
    print_test_result("String processing", true);

    // Exercise 5
    struct NumberSet {
        numbers: HashSet<i32>,
    }
    impl NumberSet {
        fn new() -> Self {
            Self {
                numbers: HashSet::new(),
            }
        }
        fn add(&mut self, n: i32) {
            self.numbers.insert(n);
        }
        fn remove(&mut self, n: i32) {
            self.numbers.remove(&n);
        }
        fn contains(&self, n: i32) -> bool {
            self.numbers.contains(&n)
        }
        fn sum(&self) -> i32 {
            self.numbers.iter().sum()
        }
    }
    let mut set = NumberSet::new();
    set.add(1);
    set.add(2);
    set.add(3);
    println!("Set contains 2: {}", set.contains(2));
    println!("Sum of numbers: {}", set.sum());
    print_test_result("Custom collection", true);
}
*/ 