use std::io;

enum Operation { 
    Add(f64, f64),
    Subtract(f64, f64), 
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(ops: Operation) -> f64 { 
    match ops {    
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() { 
    println!("Enter the first number:"); 
    let first_num = get_user_input(); 
    
    println!("Enter the operation (+, -, *, /):"); 
    let operation = get_operation(); 

    println!("Enter the second number:"); 
    let second_num = get_user_input(); 

    let op = match operation { 
        Operation::Add(a, b) => Operation::Add(first_num, second_num),
        Operation::Subtract(a, b) => Operation::Subtract(first_num, second_num),
        Operation::Multiply(a, b) => Operation::Multiply(first_num, second_num),
        Operation::Divide(a, b) => Operation::Divide(first_num, second_num),
      };

    let result = calculate(op); 

    println!("Result: {}", result); 
}

fn get_user_input() -> f64 { 
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: f64 = input.trim().parse().expect("Invalid number");
    input
}

fn get_operation() -> Operation { 
    let mut op = String::new();
    io::stdin()
        .read_line(&mut op)
        .expect("Failed to read operation");

    let op = op.trim();
    let operation = match op { 
        "+" => Operation::Add(0.0, 0.0),
        "-" => Operation::Subtract(0.0, 0.0),
        "*" => Operation::Multiply(0.0, 0.0),
        "/" => Operation::Divide(0.0, 0.0),
        _ => panic!("Invalid operation"),
    };

    operation
}
