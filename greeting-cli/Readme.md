# Greeting CLI

## Concepts Covered

- **Basic input/output**
- **Printing**
- **Variables**

## Project Overview

This project demonstrates:

- Using the `println!` macro for formatted output and string manipulation.
- Reading command-line input.
- Working with mutable variables and formatting output.
- Leveraging Rust’s `println!` macro (from `std::fmt`) to print to the console with compile-time formatting checks.

## Concepts

The concepts used in the project are:

- Basic input/output : used Module io
- println! macro for the output.

## How are macros different from functions

- macros are expanded by the compiler before the code is run, essentially performping the 'find and replace' operation at the compile time.
- This allows macros to handle a variable number of arguments and perform more complex operations that would be difficult or impossible for functions. 
- macros are a powerful way to generate code at compile time, enabling metaprogramming and code reuse.