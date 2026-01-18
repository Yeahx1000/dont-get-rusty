//! Unsafe Rust
//! This module contains exercises to help you understand:
//! - Unsafe blocks and functions
//! - Raw pointers
//! - Unsafe traits
//! - FFI
//! - Memory safety

// Exercise 1: Unsafe Functions
// TODO: Write an unsafe function that:
// - Takes a raw pointer
// - Dereferences it safely
// - Returns the value
// Handle null pointers

// Exercise 2: Raw Pointers
// TODO: Write a function that:
// - Creates a raw pointer
// - Modifies the value through the pointer
// - Converts back to safe references
// Use proper safety checks

// Exercise 3: Unsafe Traits
// TODO: Create a trait that:
// - Is marked as unsafe
// - Has unsafe methods
// - Requires safety invariants
// Document safety requirements

// Exercise 4: Memory Management
// TODO: Write a function that:
// - Allocates memory manually
// - Initializes it with values
// - Frees it properly
// Handle allocation failures

// Exercise 5: Safe Wrapper
// TODO: Create a safe wrapper around:
// - Unsafe code
// - Raw pointers
// - Memory operations
// Implement Drop trait

// Bonus Challenge:
// TODO: Create a custom allocator that:
// - Manages memory blocks
// - Handles fragmentation
// - Provides safe interface
// Implement GlobalAlloc
