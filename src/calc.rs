fn calc_solve(problem: String, math_op: &str) {
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

pub fn calc_run(input: &str) {
    let problem = input.split(' ').collect::<Vec<&str>>()[1].trim().to_string();
    if problem.contains('x') {
        calc_solve(problem, "x");
    } else if problem.contains('/') {
        calc_solve(problem, "/");
    } else if problem.contains('+') {
        calc_solve(problem, "+");
    } else if problem.contains('-') {
        calc_solve(problem, "-");
    }
}
