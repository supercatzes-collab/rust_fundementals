use std::io::{self, Write};

pub fn raw_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("raw_input failed.");
    input.trim().to_string()
}

pub fn get_input_string(prompt: &str) -> String {
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("get_input_string failed.");
        let string = raw_input();
        if string.is_empty() {
            println!("String cannot be empty {string}");
            continue;
        }
        return string;
    }
}

pub fn parser_i32(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.trim().parse::<i32>() 
}

pub fn get_i32(prompt: &str) -> i32 {
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("get_i32 failed.");
        let input = raw_input();
        match parser_i32(&input) {
            Ok(num) => return num,
            Err(_) => println!("get_i32 err")
        }
    }
}

//this function is temporary
pub fn get_f64(prompt: &str) -> f64 {
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("get_f64 failed.");
        let input = raw_input();
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number, try again."),
        }
    }
}