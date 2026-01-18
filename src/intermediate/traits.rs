//! Traits and Generics in Rust
//! This module contains exercises to help you understand:
//! - Trait definitions and implementations
//! - Generic types and functions
//! - Trait bounds
//! - Associated types
//! - Default implementations

// Exercise 1: Basic Trait
// TODO: Create a trait named 'Printable' that:
// - Has a method to print the value
// - Has a default implementation
// - Implement it for different types

// Exercise 2: Generic Functions
// TODO: Write a function that:
// - Takes a vector of any type that implements Ord
// - Returns the maximum value
// - Returns Option<T> to handle empty vectors

// Exercise 3: Trait Bounds
// TODO: Create a struct that:
// - Can store any type that implements Display
// - Has a method to print the stored value
// - Has a method to update the value

// Exercise 4: Associated Types
// TODO: Create a trait named 'Container' that:
// - Has an associated type for the item
// - Has methods to add and remove items
// - Implement it for different collection types

// Exercise 5: Multiple Trait Bounds
// TODO: Write a function that:
// - Takes two values of the same type
// - Returns their sum
// - Works with any type that implements:
//   - Add
//   - Clone
//   - Display

// Bonus Challenge:
// TODO: Create a trait that:
// - Has a method to convert between types
// - Uses associated types
// - Has a default implementation
// Implement it for common type conversions
