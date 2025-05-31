use std::io;

fn main() {
    println!("welcome basic calculator");
    println!("Choose an operation: ADD, SUB, MUL, DIV");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("error while reading the operation");
    
    println!("enter the first operand");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("error while reading first operand");
    
    println!("enter the second operand");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("error while reading second operand");
    
    let op1: f64 = num1.trim().parse().expect("invalid value");
    let op2: f64 = num2.trim().parse().expect("invalid value");
    
    match choice.trim().to_uppercase().as_str(){
        "ADD" => {
            let ans = add(op1, op2);
            println!("ADDITION of {} and {} gives {}", op1, op2, ans);
        }
        "SUB" => {
            let ans = sub(op1, op2);
            println!("SUBTRACTION of {} and {} gives {}", op1, op2, ans);
        }
        "DIV" => {
            if op2 == 0.0 {
                println!("DIVISION by ZERO is not possible");
                return;
            }
            let ans = divide(op1, op2);
            println!("DIVISION of {} by {} gives {}", op1, op2, ans);
            
        }
        "MUL" => {
            let ans = multiply(op1, op2);
            println!("MULTIPLICATION of {} by {} gives {}", op1, op2, ans);
        }
        _ => {
            println!("invalid choice");
            return;
        }
    }

}

fn add(op1:f64, op2:f64) -> f64 {
    op1 + op2
}

fn sub(op1:f64, op2:f64) -> f64 {
    op1 - op2
}

fn divide(op1:f64, op2:f64) -> f64 {
    op1 / op2
}

fn multiply(op1:f64, op2:f64) -> f64 {
    op1 * op2
}





