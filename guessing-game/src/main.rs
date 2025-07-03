//Guessing game
use std::io;
use rand::Rng; // Import the random number generator

fn main() {
    let q = 100; // Define a constant for the exit command
    println!("Welcome to the Guessing Game!");
    println!("Try to guess the number I'm thinking of between 0 and 100."); 
    println!("Enter {} to exit the game at any time.", q);
    loop{
        
        let mut rng = rand::rng(); // Create a random number generator
        let number: u32 = rng.random_range(0..=100); // Generate a random number between 0 and 100
        println!("Enter your guess (0-100):");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let guess: u32 = input.trim().parse().expect("Please enter a number");
        match guess {
            _ if guess < number => println!("Too low!"),
            _ if guess > number => println!("Too high!"),
            _ => println!("You guessed it!"),
        }
    println!("The number was: {}", number);

    if guess == q {
        println!("Exiting the game. Thanks for playing!");
        break;
    } else {
        println!("Let's play again!");              
    }
    }
}
