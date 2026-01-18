//! Ownership, Borrowing, and Lifetimes in Rust
//! This module contains exercises to help you understand:
//! - Ownership rules
//! - Borrowing (mutable and immutable)
//! - Lifetimes
//! - References and borrowing

// Exercise 1: Basic Ownership
// TODO: Write a function that:
// - Takes ownership of a String
// - Prints it
// - Returns it
// Try calling it twice with the same string

// Exercise 2: Mutable References
// TODO: Write a function that:
// - Takes a mutable reference to a String
// - Appends " world" to it
// - Returns nothing (unit type)
// Try calling it with the same string multiple times

// Exercise 3: Multiple References
// TODO: Write a function that:
// - Takes a vector of integers
// - Returns both the first and last elements
// - Uses references to avoid copying
// Handle the case where the vector is empty

// Exercise 4: Lifetimes
// TODO: Write a struct named 'TextWindow' that:
// - Has a reference to a string slice
// - Has a start and end position
// - Implements a method to get the text between positions
// - Uses lifetime annotations

// Exercise 5: Ownership and Move Semantics
// TODO: Write a function that:
// - Takes ownership of a Vec<i32>
// - Removes all even numbers
// - Returns the modified vector
// Try to use the original vector after calling the function

// Bonus Challenge:
// TODO: Write a function that:
// - Takes a reference to a string
// - Returns a new string containing only the vowels
// - Uses iterators and collect
