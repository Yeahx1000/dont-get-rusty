# Don't Get Rusty 🦀 <!-- omit in toc -->

A structured Rust learning project designed to help you progress from beginner to advanced concepts in Rust programming.

Slapped this together awhile back for personal use and forgot about it, so it's not perfect (will possibly add more over time), it could be helpful, so sharing. Decent for practicing some Rust concepts, but not a replacement for a real project and real world experience.

- [Helpful Links and Tools](#helpful-links-and-tools)
- [Difficulty Levels](#difficulty-levels)
  - [Beginner](#beginner)
  - [Intermediate](#intermediate)
  - [Advanced](#advanced)
  - [Bonus Level](#bonus-level)
- [How to Use This Project](#how-to-use-this-project)
- [Answer Guides](#answer-guides)
- [Getting Started](#getting-started)
- [Tips for Learning](#tips-for-learning)
- [Contributing](#contributing)
- [License](#license)

## Helpful Links and Tools

These are some helpful links and tools I've found useful.

- ["The Book" 🦀](https://doc.rust-lang.org/book/)
- [Rust Lang Cheatsheet](https://cheats.rs/)
- [Rust Developer Roadmap](https://roadmap.sh/rust)

## Difficulty Levels

This project is organized into three main sections:

### Beginner

- Variables and basic types
- Control flow (if/else, loops, match)
- Functions and closures

### Intermediate

- Ownership, borrowing, and lifetimes
- Collections (Vectors, HashMaps)
- Error handling with Result and Option
- Traits and generics

### Advanced

- Concurrency and async/await
- Macros and metaprogramming
- Unsafe Rust
- Foreign Function Interface (FFI)

### Bonus Level

Here will just be random excercises on more obscure concepts, just because.

## How to Use This Project

1. Start with the beginner exercises in `src/beginner/`
2. Complete each module in order
3. Try to solve the exercises without looking at the solutions
4. Progress to intermediate and advanced concepts as you become comfortable

## Answer Guides

If you get stuck or want to verify your solutions, answer guides are available in each section:

- Beginner answers: `src/beginner/answers/answers.md`
- Intermediate answers: `src/intermediate/answers/answers.md`
- Advanced answers: `src/advanced/answers/answers.md`

Each answer guide contains complete solutions with explanations. However, it's recommended to:

1. Try solving the exercises yourself first
2. Only look at the answers when you're stuck
3. Use the answers to understand different approaches
4. Experiment with modifying the solutions

## Getting Started

1. Make sure you have Rust installed (<https://rustup.rs/>)
2. Clone this repo
3. Run `cargo build` to ensure everything compiles
4. Start with `src/beginner/variables.rs` prefereably.

## Tips for Learning

- Read the comments in each file carefully
- Try to modify the examples to see what happens
- When stuck, use `cargo doc --open` to read the documentation
- Experiment with the code as you see fit,  make mistakes, that's how you learn
- If stuck, check the answer guides (but try to understand the solutions)

There's no one way to answer these, do what works for you, but try to be mindful of general language best practices for the future.

## Contributing

Feel free to contribute improvements or additional exercises by submitting pull requests.

## License

This project is open source and available under the MIT License.
