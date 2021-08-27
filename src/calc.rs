fn calc_solve(problem: &str, math_op: &str) {
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

pub fn calc_run(problem: &str) {
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

pub fn calc_return(problem: &str) -> i32 {
    let math_op = if problem.contains('x') {
        "x"
    } else if problem.contains('/') {
        "/"
    } else if problem.contains('+') {
        "+"
    } else if problem.contains('-') {
        "-"
    } else {
        ""
    };
    let problem_vector = problem.split(math_op).collect::<Vec<&str>>();
    let first_number = problem_vector[0].parse::<i32>().unwrap();
    let second_number = problem_vector[1].parse::<i32>().unwrap();
    match math_op {
        "x" => return first_number * second_number,
        "/" => return first_number / second_number,
        "+" => return first_number + second_number,
        "-" => return first_number - second_number,
        _ => return 123456789,
    }
}

