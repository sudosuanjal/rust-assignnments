use std::io;


fn main() {
    println!("welcome to string reverser");
    let mut word = String::new();
    println!("enter the string to reverse");
    io::stdin().read_line(&mut word).expect("error while reading the string");
    let rev_string: String = word.trim().chars().rev().collect();
    
    println!("the entered string is: {}", word);
    println!("the reversed string is: {}", rev_string);
    
}
