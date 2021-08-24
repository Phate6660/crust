use std::io::Write;
use std::process::Command;

fn display_prompt() {
    let crusty_prompt = String::from("[crusty]: ");
    let prompt = std::env::var("PROMPT").unwrap_or(crusty_prompt);
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
}

fn parse_input(op: &str) -> String {
    if op == "interactive" {
        let mut input = std::string::String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read user input");
        input.trim().to_string()
    } else {
        std::env::args().collect::<Vec<String>>()
            .get(2).unwrap()
            .replace('"', "")
            .trim().to_string()
    }
}
fn calc(problem: String, math_op: &str) {
    let problem_vector = problem.split(math_op).collect::<Vec<&str>>();
    let first_number = problem_vector[0].parse::<i32>().unwrap();
    let second_number = problem_vector[1].parse::<i32>().unwrap();
    match math_op {
        "x" => println!("{}", first_number * second_number),
        "/" => println!("{}", first_number / second_number),
        "+" => println!("{}", first_number + second_number),
        "-" => println!("{}", first_number - second_number),
        _ => println!("Error, '{}' is an unsupported operation.", math_op),
    }
}


fn calc_main(input: &str) {
    let problem = input.split(' ').collect::<Vec<&str>>()[1].trim().to_string();
    if problem.contains('x') {
        calc(problem, "x");
    } else if problem.contains('/') {
        calc(problem, "/");
    } else if problem.contains('+') {
        calc(problem, "+");
    } else if problem.contains('-') {
        calc(problem, "-");
    }
}

fn run_command(input: String) {
    if input.starts_with("calc") {
        calc_main(&input);
    } else if input.contains(' ') {
        let input = input.split(' ').collect::<Vec<&str>>();
        let child = Command::new(input[0])
            .args(&input[1..])
            .spawn()
            .or(Err(()));
        if child.is_err() {
            println!("Sorrry, '{}' was not found!", input[0]);
        } else {
            child.unwrap().wait().unwrap();
        }
    } else {
        let child = Command::new(&input)
            .spawn()
            .or(Err(()));
        if child.is_err() {
            println!("Sorry, '{}' was not found!", input);
        } else {
            child.unwrap().wait().unwrap();
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let na = String::from("no args");
    if args.get(1).unwrap_or(&na) == "-c" {
        let input = parse_input("non-interactive");
        run_command(input);
        std::process::exit(0);
    }
    loop {
        display_prompt();
        let input = parse_input("interactive");
        run_command(input);
    }
}
