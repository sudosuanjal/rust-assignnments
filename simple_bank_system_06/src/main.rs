use std::io;

enum  Transaction {
    Deposit(f64),
    Withdraw(f64),
}

struct  BankAccount{
    balance:f64,
    acc_no:String,
}
fn main() {
    println!("Hello, Welcome to The Simple Bank System");
    println!("enter a sample back acc namem like ACC-123");
    let mut acc_name = String::new();
    io::stdin().read_line(&mut acc_name).expect("error while reading the account name");
    let acc_name = acc_name.trim();
    println!("your back account initialised with acc no: {}", acc_name);

}
