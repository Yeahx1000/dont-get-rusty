//! Concurrency in Rust
//! This module contains exercises to help you understand:
//! - Threading
//! - Message passing
//! - Shared state
//! - Async/await
//! - Parallel processing

#[allow(unused_imports)]
use crate::utils::{
    create_test_async_tasks, create_test_numbers, create_test_vector, print_section_header,
};

// Exercise 1: Basic Threading
// TODO: Write a function that:
// - Creates multiple threads
// - Each thread prints a message
// - Waits for all threads to complete
// Use std::thread

// Exercise 2: Message Passing
// TODO: Write a function that:
// - Creates a channel
// - Spawns a producer thread
// - Spawns a consumer thread
// - Uses mpsc channel
// Handle errors gracefully

// Exercise 3: Shared State
// TODO: Write a function that:
// - Creates a shared counter
// - Spawns multiple threads
// - Each thread increments the counter
// - Uses Arc and Mutex
// Handle errors gracefully

// Exercise 4: Async/Await
// TODO: Write a function that:
// - Creates multiple async tasks
// - Each task sleeps for a duration
// - Waits for all tasks to complete
// Use tokio

// Exercise 5: Parallel Processing
// TODO: Write a function that:
// - Takes a vector of numbers
// - Squares each number in parallel
// - Returns the sum of squares
// Use rayon

// Bonus Challenge:
// TODO: Write a function that:
// - Creates a thread pool
// - Processes tasks in parallel
// - Returns results in order
// Use tokio::spawn and futures

