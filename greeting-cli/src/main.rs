use std::io;

fn main() {
    let mut name = String::new();
    println!("Please enter your name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line"); 

    let name = name.trim(); // Trim whitespace
    if name.is_empty() {
        println!("Hello, World!");
    } else {        
        println!("Hello, {}!", name);   
    }
}