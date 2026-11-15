use std::io;

fn take_input() -> String {
    let mut input: String = String::new();
    
    io::stdin()
    .read_line(&mut input)
    .expect("Invalid input");
    
    input.trim().to_string()
}

fn get_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let input = take_input();
        
        // Try to parse the input
        match input.parse::<f64>() {
            Ok(num) => return num, // If it works, return the number and exit the function/loop
            Err(_) => {
                println!("Invalid input!");
                std::process::exit(0)
            }
        }
    }
}

fn calculate(n1: f64, n2: f64, operator: &str) -> Option<f64> {
    match operator {
        "+" => Some(n1 + n2),
        "-" => Some(n1 - n2),
        "*" => Some(n1 * n2),
        "/" => Some(n1 / n2),
        "%" => Some(n1 % n2),
        _ => {
            println!("Unknown operator!");
            None
        }
    }
}

fn main() {
    let num1: f64 = get_number("Number 1:");
    let num2: f64 = get_number("Number 2:");

    println!("Operation:");
    let operation = take_input();

    match calculate(num1, num2, &operation) {
        Some(result) => println!("{num1} {operation} {num2} = {result}"),
        None => println!("Calculation failed."),
    }
}
