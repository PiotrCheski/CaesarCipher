use std::io;
mod encrypt;
mod decrypt;

fn main() {
    loop {
        println!("Welcome to the Ceaser Cipher Tool");
        println!("Choose one option");
        println!("Type 1 for encryption");
        println!("Type 2 for decryption");
        
        let mut menu_input: String = String::new();
        println!("Enter your choice");
        io::stdin().read_line(&mut menu_input).expect("Failed to read line");

        let choice: u8 = match menu_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        match choice {
            1 => {
                encrypt::run();
            }
            2 => {
                decrypt::run();
            }
            _ => {
                println!("Invalid number");
            }
        }
    }
}
