use std::io;

fn main() {
    println!("Enter a sentence:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let longest_word = input
        .split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap_or("");
        
    println!("The longest word is: {}", longest_word);
}
