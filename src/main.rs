use std::io;

fn take_input() -> String {
    let mut input: String = String::new();
    
    io::stdin()
    .read_line(&mut input)
    .expect("error");
    
    input.trim().to_string()
}

fn calculate(n1: i32, n2: i32, operator: &str) -> i32 {
    match operator {
        "+" => n1 + n2,
        "-" => n1 - n2,
        "*" => n1 * n2,
        "/" => n1 / n2,
        "%" => n1 % n2,
        _ => {
            println!("Unknown operator!");
            0
        }
    }
}

fn main() {
    println!("Number 1:");
    let num1: i32 = take_input().parse().expect("error");
    
    println!("Number 2:");
    let num2: i32 = take_input().parse().expect("error");

    println!("Operation:");
    let operation: String = take_input();

    let result: i32 = calculate(num1, num2, &operation);

    println!("{num1} {operation} {num2} = {result}");
}
