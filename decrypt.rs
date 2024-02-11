use std::io;

pub fn run() {
    let alphabet: &str = "abcdefghijklmnopqrstuvwxyz";

    let mut text_to_decrypt: String = String::new();
    let mut shift_input: String = String::new();
    let mut decrypted_text: String = String::new();

    println!("Enter text to decrypt");
    io::stdin().read_line(&mut text_to_decrypt).expect("Failed to read line");

    println!("Enter shift");
    io::stdin().read_line(&mut shift_input).expect("Failed to read line");

    let text_to_decrypt: String = text_to_decrypt.trim().to_string().to_lowercase();

    let shift_input: u8 = match shift_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Enter a number.");
            return ;
        }
    };

    for c in text_to_decrypt.chars() {
        if let Some(index) = alphabet.find(c) {
            let new_index = (index - shift_input as usize) % 26;
            decrypted_text.push(alphabet.chars().nth(new_index).unwrap());
        } else {
            println!("Character '{}' not found in '{}'", c, alphabet);
        }
    }
    println!("{}", decrypted_text);

}