//! Foreign Function Interface (FFI)
//! This module contains exercises to help you understand:
//! - C bindings
//! - Calling C from Rust
//! - Calling Rust from C
//! - Type conversions
//! - Safety considerations

// Exercise 1: Basic C Binding
// TODO: Write a function that:
// - Binds to a C function
// - Handles type conversions
// - Returns a Rust type
// Use extern "C" block

// Exercise 2: String Conversions
// TODO: Create functions that:
// - Convert Rust String to C string
// - Convert C string to Rust String
// - Handle null termination
// Use std::ffi

// Exercise 3: Struct Conversions
// TODO: Create a struct that:
// - Matches a C struct layout
// - Has safe conversion methods
// - Handles alignment
// Use #[repr(C)]

// Exercise 4: Callback Functions
// TODO: Write code that:
// - Registers a Rust callback
// - Calls it from C
// - Handles data passing
// Use function pointers

// Exercise 5: Error Handling
// TODO: Create a safe wrapper that:
// - Converts C error codes
// - Returns Result type
// - Provides error messages
// Handle all error cases

// Bonus Challenge:
// TODO: Create a complete FFI interface that:
// - Exports Rust functions to C
// - Handles complex types
// - Provides documentation
// Use cbindgen
