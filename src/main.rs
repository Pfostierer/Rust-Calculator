use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: {} <num>", args[0]);
        return;
    }

    let num1: i32 = args[1].parse().unwrap();
    let operator: char = args[2].chars().nth(0).unwrap();
    let num2: i32 = args[3].parse().unwrap();

    let result = calculate(num1, num2, operator);

    println!("{} {} {} = {}", num1, operator, num2, result);
}

fn calculate(num1: i32, num2: i32, operator: char) -> i32 {
    match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => panic!("Unknown operator: {}", operator),
    }
}