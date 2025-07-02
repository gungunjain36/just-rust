use std::io;


fn main() {
    Let mut counter = 0;

    Loop {
        println!("Current counter value is: {}", counter);
        println!("Enter a command (increment, decrement, reset, exit):");
        Let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        Let command = input.trim();

        match command{
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
