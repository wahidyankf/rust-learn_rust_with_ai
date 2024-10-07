# Rust Concepts Demo

This repository demonstrates important concepts in Rust programming language. Each concept is explored with practical examples to help developers understand and master these fundamental Rust features.

## Concepts Covered

1. **Ownership**

   - Understanding Rust's unique approach to memory management
   - Move semantics and borrowing rules

2. **Borrowing**

   - Shared and mutable references
   - Preventing data races at compile-time

3. **Lifetimes**

   - Explicit lifetime annotations
   - Lifetime elision rules

4. **Pattern Matching**

   - Match expressions
   - Destructuring in let statements

5. **Traits**

   - Defining and implementing traits
   - Trait bounds and trait objects

6. **Concurrency and Parallelism**

   - Threads and message passing
   - Shared-state concurrency

7. **Error Handling**

   - Result and Option types
   - The ? operator and propagating errors

8. **Macros**

   - Declarative macros
   - Procedural macros

9. **Modules & Crates**

   - Organizing code with modules
   - Creating and using external crates

10. **Unsafe Code**
    - Raw pointers and unsafe functions
    - FFI (Foreign Function Interface)

Each concept is implemented in its own module, allowing for focused exploration and learning.

## How to Run the Code

To run examples for a specific concept, use the command:

`cargo run -- <concept>`

Replace `<concept>` with the number or name of the concept you want to explore. For example:

`cargo run -- 01` or `cargo run -- ownership`
