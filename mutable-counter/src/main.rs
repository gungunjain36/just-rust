use std::io;    // Import the standard input/output library


fn main() {
    let mut counter = 0;   // Initialize a mutable counter variable

    loop {                                                                          // Start an infinite loop to continuously prompt the user
        println!("Current counter value is: {}", counter);
        println!("Enter a command (increment, decrement, reset, exit):");
        // Prompt the user for input
        let mut input = String::new();
        // Read user input from the standard input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        // Trim whitespace and convert input to lowercase for easier command matching
        let command = input.trim().to_lowercase();

        // Match the command against known actions
        // Increment, decrement, reset, or exit the program
        match command.as_str() {
            "increment" => {
                counter += 1;
                println!("Counter incremented.");
            }
            "decrement" => {
                if counter > 0 {
                    counter -= 1;
                    println!("Counter decremented.");
                } else {
                    println!("Counter cannot go below zero.");
                }
            }
            "reset" => {
                counter = 0;
                println!("Counter reset to zero.");
            }
            "exit" => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Unknown command. Please try again.");
            }
        }
    }
}
