use std::io;

#[derive(Debug)]
struct Student{
    name: String,
    age: u64,
    grade: char
}

fn main() {
    
    let mut students : Vec<Student> = Vec::new();
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
                let student = create_student();
                students.push(student);
            }
            "2" => {
                update_student(&mut students);
            }
            "3" => {
                display_students(&mut students);
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

fn create_student() -> Student{
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
    let grade : char = grade.trim().chars().next().expect("error while parsing");
    
    Student{
        name: name.trim().to_string(),
        age,
        grade,
    }
    
}

fn display_students(students:&mut Vec<Student>){
    println!("current students:");
    for (i, student) in students.iter().enumerate(){
        println!("{}. Name: {} || Age: {} || Grade: {}.",i+1,student.name, student.age, student.grade);
    }
}

fn update_student(students:&mut Vec<Student>){
    display_students(students);
    println!("enter the name of the student you want to edit");
    let mut stud_name = String::new();
    io::stdin().read_line(&mut stud_name).expect("error while reading name");
    let stud_name = stud_name.trim();
    
    let stud_index = students.iter().position(|student| student.name == stud_name);
    
    match stud_index{
        Some(i) => {
            println!("student with name: {} found",students[i].name);
        }
        None => println!("student with name: {} not found", stud_name),
    }
    
    
}
