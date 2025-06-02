use std::io;

struct Student{
    name: String,
    age: u64,
    grade: char
}

fn main() {
    println!("welcome to student management system!");
    loop{
        println!("enter the option number");
        println!("1. Enter student details to the system");
        println!("2. Update student details");
        println!("3. Print Student");
        println!("4. Quit");
        
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("error while reading the choice");
        
        match choice.trim() {
            "1" => {
                println!("you entered one");
            }
            "2" => {
                println!("you entered two");
            }
            "3" => {
                println!("you entered three");
            }
            "4" => {
                println!("you entered quit");
                break;
            }
            _ => {
                println!("invalid choice");
            }
        }
        
    }

}

fn create_student () -> Student{
    let mut name = String::new();
    let mut age = String::new();
    let mut grade = String::new();
    
    println!("enter the name of the student");
    io::stdin().read_line(&mut name).expect("error while reading the name");
    
    println!("enter the age of the student");
    io::stdin().read_line(&mut age).expect("error while reading the age");
    
    println!("enter the grade of the student");
    io::stdin().read_line(&mut grade).expect("error while reading the grade");
    
    let age : u64 = age.trim().parse().expect("error while parsing");
    let grade : char = age.trim().chars().next().expect("error while parsing");
    
    Student{
        name: name.trim().to_string(),
        age,
        grade,
    }
    
}
