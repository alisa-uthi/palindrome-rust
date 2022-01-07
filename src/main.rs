use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a word that you want to check for palindrome : ");
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    check_palindrome(input);
}

fn check_palindrome(input: String) {
    let mut palindrome: bool = true;
    let word: String = input.trim().to_lowercase().to_string();
    
    for (index, letter) in word.chars().enumerate() {
        // Loop through half of the input
        if index == word.len() / 2 {
            break;
        }

        // If the current letter and a letter at the same distance from the end of the input is not the same
        if letter != word.chars().nth(word.len() - index - 1).unwrap() {
            palindrome = false;
            break;
        }
    }

    if palindrome {
        println!("{} is a palindrome.", input.trim());
    } else {
        println!("{} is not a palindrome.", input.trim());
    }
}