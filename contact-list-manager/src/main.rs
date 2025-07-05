use std::io;

fn main() {
    println!("Welcone to the Contact List Manager!");
    // defining a struct to represent a contact
    struct Contact {
        name: String,
        phone: String,
        email: String,
    }
    // creating a vector to store contacts
    // A vector is a growable array type in Rust, which can be used to store
    // a list of contacts dynamically.
    // It allows us to add, remove, and find contacts easily.
    // We will use this vector to store all the contacts added by the user.

    let mut contacts: Vec<Contact> = Vec::new();

    // The main loop of the program
    // This loop will continue until the user chooses to exit the program.
    // It will display a menu of options for the user to choose from,
    // allowing them to add, remove, or find contacts.
    // The user can also exit the program at any time.
    // The loop will read user input and perform the corresponding action based on the user's choice.

    loop {
        println!("Please choose an option:");
        println!("1. Add a contact");
        println!("2. Remove a contact");
        println!("3. Find a contact");
        println!("4. Exit");

        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                let mut name = String::new();
                let mut phone = String::new();
                let mut email = String::new();

                println!("Enter contact name:");
                io::stdin().read_line(&mut name).expect("Failed to read line");
                println!("Enter contact phone:");
                io::stdin().read_line(&mut phone).expect("Failed to read line");
                println!("Enter contact email:");
                io::stdin().read_line(&mut email).expect("Failed to read line");

                let contact = Contact {
                    name: name.trim().to_string(),
                    phone: phone.trim().to_string(),
                    email: email.trim().to_string(),
                };

                contacts.push(contact);
                println!("Contact added successfully!");
            }
            2 =>{
                let mut name = String::new();
                println!("Enter the name of the contact to remove:");
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();

                for i in 0..contacts.len() {
                    if contacts[i].name == name {
                        contacts.remove(i);
                        println!("Contact removed successfully!");
                        break;
                    }
                }
            }

            3=>{
                let mut name = String::new();
                println!("Enter the name of the contact to find:");
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();

                for contact in &contacts {
                    if contact.name == name {
                        println!("Found contact: {} {} {}", contact.name, contact.phone, contact.email);
                        break;
                    }
                }
            }

            4 => {
                println!("Exiting the Contact List Manager. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }

        }
    }
}
