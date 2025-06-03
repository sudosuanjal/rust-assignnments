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
        println!("2. Update/Delete student details");
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
    println!("enter the name of the student you want to edit/delete");
    let mut stud_name = String::new();
    io::stdin().read_line(&mut stud_name).expect("error while reading name");
    let stud_name = stud_name.trim();
    
    let stud_index = students.iter().position(|student| student.name == stud_name);
    
    match stud_index{
        Some(i) => {
            println!("student with name: {} found",students[i].name);
            println!("what you want to edit (1.NAME -- 2.AGE -- 3.GRADE -- 4.DELETE STUDENT");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("error while reading the input");
            match choice.trim(){
                "1" => {
                    let mut new_name = String::new();
                    println!("enter the new name:");
                    io::stdin().read_line(&mut new_name).expect("error while reading the new name");
                    let new_name = new_name.trim();
                    students[i].name = new_name.to_string();
                }
                "2" => {
                    println!("enter the new age");
                    let mut new_age = String::new();
                    io::stdin().read_line(&mut new_age).expect("error while reading new age");
                    let new_age : u64 = new_age.trim().parse().expect("error while parsing the new age");
                    students[i].age = new_age;
                }
                "3" =>  {
                    let mut new_grade = String::new();
                    println!("enter the new grade");
                    io::stdin().read_line(&mut new_grade).expect("error while reading the new grade");
                    let new_grade = new_grade.trim();
                    students[i].grade = new_grade.chars().next().expect("error while parsing to char");
                }
                "4" =>{
                    students.remove(i);
                    println!("student with name: {} deleted from the system", stud_name);
                }
                
                _ => println!("invalid input"),
            }
        }
        None => println!("student with name: {} not found", stud_name),
    }
    
    
}
