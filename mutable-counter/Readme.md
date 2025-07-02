# Mutable Counter

This is a simple Rust program that repeatedly prompts the user to either increment or reset a counter. The program demonstrates basic input/output handling, mutable state, and control flow in Rust.

## Features

- Continuously displays the current counter value.
- Allows the user to increment the counter.
- Provides an option to reset the counter to zero.
- Gracefully handles invalid input.

## Usage

1. Clone the repository:
    ```sh
    git clone https://github.com/your-username/mutable-counter.git
    cd mutable-counter
    ```

2. Build and run the program:
    ```sh
    cargo run
    ```

3. Follow the on-screen prompts to increment or reset the counter.

## Example

```
Current counter: 0
Type 'i' to increment, 'r' to reset, or 'q' to quit: i
Current counter: 1
Type 'i' to increment, 'r' to reset, or 'q' to quit: r
Current counter: 0
Type 'i' to increment, 'r' to reset, or 'q' to quit: q
Goodbye!
```

## Concepts

- loop : A loop keyword to indicate an infinite loop.
        - The break statement can be used to exit a loop at anytime, whereas the continue statement can be used to skip the rest of the iteration and start a new one.
- match : Rust provides pattern matching via the match keyword, which can be used like a C switch.