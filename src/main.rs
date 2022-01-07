use std::io;
use regex::Regex;

fn main() {
    let mut input = String::new();
    println!("Enter a word that you want to check for palindrome : ");
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    check_palindrome(input);
}

fn check_palindrome(input: String) {
    // Format input
    let formatted_input: String = input.trim().to_lowercase();
    let word: &str = formatted_input.as_str();

    // Remove special characters
    let regex = Regex::new(r"[^a-zA-Z0-9ก-๛]").unwrap();
    let word = &regex.replace_all(word, "");

    let mut palindrome: bool = true;
    
    for (index, letter) in word.chars().enumerate() {
        // Loop through half of the input
        if index == word.len() / 2 {
            break;
        }
        
        // If the current letter and a letter at the same distance from the end of the input is not the same
        if letter != word.chars().rev().nth(index).unwrap() {
            palindrome = false;
            break;
        }
    }

    if palindrome {
        println!("\"{}\" is a palindrome.", input.trim());
    } else {
        println!("\"{}\" is not a palindrome.", input.trim());
    }
}