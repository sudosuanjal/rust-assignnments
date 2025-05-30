use std::io;

fn main() {
    println!("Hello, Welcome to Even or Odd Detector");
    
    let mut value = String::new();
    println!("enter the number");
    io::stdin().read_line(&mut value).expect("an error occurred while reading the number");
    let number:i32 = value.trim().parse().expect("an error occurred while parsing");
    println!();
    println!("the entered number is {}",number);
    let ans = number % 2;
    if ans == 0{
        println!("{} is even!", number);
    }else{
        println!("{} is odd", number);
    }
    
}
