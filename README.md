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
- [Project Structure](#project-structure)
- [Tips for Learning](#tips-for-learning)
- [Running Examples](#running-examples)
- [Testing](#testing)
- [Contributing](#contributing)
  - [Adding New Exercises](#adding-new-exercises)
  - [Improving Existing Content](#improving-existing-content)
  - [Submission Process](#submission-process)
  - [Code Style](#code-style)
- [Dependencies](#dependencies)
- [License](#license)
- [Support](#support)

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

1. **Install Rust**: Make sure you have Rust installed (<https://rustup.rs/>)

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone this repo**

   ```bash
   git clone https://github.com/your-username/dont-get-rusty.git
   cd dont-get-rusty
   ```

3. **Build and run**

   ```bash
   cargo build    # Build the project
   cargo run      # Run the examples
   cargo test     # Run the tests
   ```

4. **Start learning**: Begin with `src/beginner/variables.rs` and work your way up!

## Project Structure

```text
src/
├── beginner/          # Basic Rust concepts
├── intermediate/      # Advanced Rust features
├── advanced/          # Concurrency, macros, FFI
├── bonus/             # Additional challenging exercises
└── utils/             # Shared utility functions
```

## Tips for Learning

- Read the comments in each file carefully
- Try to modify the examples to see what happens
- When stuck, use `cargo doc --open` to read the documentation
- Experiment with the code as you see fit, make mistakes, that's how you learn
- If stuck, check the answer guides (but try to understand the solutions)

## Running Examples

The project includes practical examples using popular Rust crates:

- **Rayon**: Parallel processing examples
- **Tokio**: Async/await demonstrations
- **Serde**: Serialization examples

## Testing

Run the comprehensive test suite:

```bash
cargo test
```

All utility functions have unit tests to ensure correctness.

## Contributing

We welcome contributions! Here's how you can help:

### Adding New Exercises

1. Create a new module in the appropriate difficulty level
2. Follow the existing pattern: TODO comments for exercises, answer guides in `answers/`
3. Add tests for any new utility functions
4. Update the README with any new dependencies

### Improving Existing Content

- Fix typos or errors
- Add more detailed explanations
- Improve code examples
- Add more test cases

### Submission Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin feature/your-feature-name`
5. Submit a pull request

### Code Style

- Follow Rust naming conventions
- Use `#[allow(dead_code)]` for educational examples
- Add comments for complex concepts
- Keep exercises focused and practical

## Dependencies

This project uses several key Rust crates:

- `tokio`: Async runtime for async/await examples
- `rayon`: Data parallelism library
- `serde`: Serialization framework
- `futures`: Async programming utilities

## License

This project is open source and available under the [MIT License](LICENSE).

## Support

If you encounter issues or have questions:

- Check the answer guides first
- Review the Rust documentation
- Open an issue if you find bugs or have suggestions

Happy Rust learning! 🦀
