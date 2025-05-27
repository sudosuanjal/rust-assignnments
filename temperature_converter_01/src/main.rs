use std::io;

fn main(){
    println!("welcome to temperature converter");
    
    loop{
        println!("enter 'C' to convert from Celsius to Fahrenheit");
        println!("enter 'F' to convert from Fahrenheit to Celsius");
        println!("enter 'Q' for exiting the menu");
    
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("an error occurred while reading");
        
        let mut value = String::new();
        
        match choice.trim(){
            "C" => {
                println!("enter the temperature in celsius");
                io::stdin().read_line(&mut value).expect("an error occurred while reading celsius");
                let temp: f64 = value.trim().parse().expect("invalid float");
                let fah = c_to_f(temp);
                println!("{}°C is {}°F", temp, fah);
            }
            
            "F" => {
                println!("enter the temperature in fahrenheit");
                io::stdin().read_line(&mut value).expect("an error occurred while reading fahrenheit");
                let temp: f64 = value.trim().parse().expect("invalid float");
                let cel = f_to_c(temp);
                println!("{}°F is {}°C", temp, cel);
            }
            
            "Q" =>{
                println!("Exiting bye...");
                break;
            }
            
            _ => {println!("invalid choice");}
        }
    }
}

fn c_to_f(temp: f64) -> f64{
    (temp * 9.0 / 5.0) + 32.0
}

fn f_to_c(temp: f64) -> f64{
    (temp - 32.0) * 5.0 / 9.0
}
