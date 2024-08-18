use std::io;
#[allow(dead_code)]
#[allow(unused_variables)]


enum Operation{
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
}


fn calculate(operation: Operation) -> f64{
    match operation {
        Operation::Add(a, b) => {
            return a + b;
        },
        Operation::Subtract(a, b) => {
            return a - b;
        },
        Operation::Multiply(a, b) => {
            return a * b;
        },
        Operation::Divide(a, b) => {
            if b!= 0.0
            {
                return a/b;
            }
            else{
                println!("Cannot divide by zeroo: ");
                return 0.0;
            }
        }
    }
}


fn main()
{
    println!("Enter the first number: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read first number");

    let num1: f64 = input1.trim().parse().expect("Please enter a valid number as a float");


    println!("Enter the operation (+, -, *, /):");
    let mut op_input = String::new();
    io::stdin().read_line(&mut op_input).expect("Failed to read operation input");
    let operation = op_input.trim();


    println!("Enter the second number: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to red");

    let num2: f64 = input2.trim().parse().expect("Please enter a valid number as a float");

    let operation_enum = match operation{
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => {
            println!("Invalid operation input");
            return;
        }
    };


    let result = calculate(operation_enum);
    println!("The result is: {}", result);
}