use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("Choose c or f to convert");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        let measurement = input.trim();
        let mut input = String::new();
        println!("Input a temperature");
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let temperature: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        if measurement == "f" {
            println!("{}F = {}C", temperature, convert_f_to_c(temperature));
        } else if measurement == "c" {
            println!("{}C = {}F", temperature, convert_c_to_f(temperature));
        } else {
            println!("Invalid unit selected.");
            continue;
        }
        break;
    }
}

// C = (5/9)(x-32)
fn convert_f_to_c (x: f32) -> f32 {
    (5.0/9.0) * (x-32.0)
}

// F = x(9/5)+32
fn convert_c_to_f (x: f32) -> f32 {
    x * (9.0/5.0) + 32.0
}
