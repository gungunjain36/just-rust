# Guessing Game

Welcome to the **Guessing Game**! This is a simple command-line game written in Rust where the player tries to guess a randomly generated number within a specified range.

## Features

- Random number generation within a configurable range.
- User-friendly command-line interface.
- Input validation and helpful prompts.
- Feedback after each guess (too high, too low, or correct).
- Keeps track of the number of attempts.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

### Running the Game

1. Clone the repository:
    ```sh
    git clone <repository-url>
    cd guessing-game
    ```

2. Build and run the game:
    ```sh
    cargo run
    ```

3. Follow the on-screen instructions to play.

## How to Play

- The program will randomly select a number within a given range (e.g., 1 to 100).
- Enter your guess and press Enter.
- The game will tell you if your guess is too high, too low, or correct.
- Continue guessing until you find the correct number.
- The game will display the number of attempts you took to guess correctly.


## Concepts Covered

- User input
- parsing : Parsing is the process of analyzing a string or data structure and converting it into a more usable format. For example, converting a string like "42" into the integer 42. Parsing often involves checking if the input matches expected patterns.
- match : match is a control flow construct used to compare a value against a series of patterns and execute code based on which pattern matches. It's similar to a switch statement but often more powerful, allowing for pattern matching on data structures.

- Result (error handling) : Result is a common type used for error handling. It represents either a success (Ok) or an error (Err). Using Result helps handle errors gracefully without crashing the program, by explicitly dealing with possible failure cases.