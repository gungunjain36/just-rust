# Longest Word Finder

A simple Rust project to find the longest word in a sentence using string slices, references, and lifetimes.

## Overview

This project demonstrates how to:

- Write a function that takes a sentence (`&str`) and returns the longest word as a string slice (`&str`), **without copying** the data.
- Use **string slicing** and **references** to efficiently work with parts of strings.
- Apply **lifetime annotations** to ensure that returned references are valid and safe.

## Concepts Covered

- **References**: Borrowing parts of a string without taking ownership.
- **String Slices**: Accessing substrings efficiently.
- **Lifetimes**: Ensuring that references remain valid and preventing dangling pointers.

## How to Run

1. Clone the repository.
2. Run with `cargo run` or test with `cargo test`.

## Why This Matters

Rust’s lifetimes and borrowing rules ensure memory safety and prevent bugs like dangling pointers. This project is a practical example of how to use these features in real code.

---
