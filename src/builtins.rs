extern crate term;
use std::io::prelude::*;

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
        "x" => first_number * second_number,
        "/" => first_number / second_number,
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        _ => 123456789,
    }
}

pub fn cd(input: &str) -> std::io::Result<()> {
    let path = std::path::Path::new(input);
    std::env::set_current_dir(&path)?;
    Ok(())
}

pub fn help() {
    println!("\
        cRUSTy [https://github.com/Phate6660/crusty]\n\
        builtins:\n\
        ---------\n\
        calc\n\
        cd\n\
        echo\n\
        exit\n\
        help\n\
        ls\n\
        pwd\
        ");
}

pub fn ls(input: &str) {
    let mut t = term::stdout().unwrap();

    let path;
    if std::path::Path::new(input).exists() {
        path = std::fs::read_dir(input).unwrap()
    } else {
        println!("ERROR: '{}' is not a valid file or directory.", input);
        return
    }

    for file in path {
        let raw_entry = file.unwrap().path();
        #[cfg(target_os = "linux")]
        let still_raw_entry = raw_entry.to_str().unwrap().replace("./", ""); 
        #[cfg(target_os = "windows")]
        let still_raw_entry = raw_entry.to_str().unwrap().replace(".\\", "");
        let paths = still_raw_entry.split('\n');
        for line in paths {
            #[cfg(target_os = "linux")]
            let parts = line.split('/');
            #[cfg(target_os = "windows")]
            let parts = line.split('\\');
            let mut n = 0;
            #[cfg(target_os = "linux")]
            let parts_count = line.split('/').count();
            #[cfg(target_os = "windows")]
            let parts_count = line.split('\\').count();
            for part in parts {
                if part.starts_with('.') || n == parts_count - 1 {
                    t.fg(term::color::WHITE).unwrap();
                } else {
                    t.fg(term::color::GREEN).unwrap();
                }
                write!(t, "{}", part).unwrap();
                t.reset().unwrap();
                n += 1;
                if n == parts_count {
                    break;
                } else {
                    #[cfg(target_os = "linux")]
                    write!(t, "/").unwrap();
                    #[cfg(target_os = "windows")]
                    write!(t, "\\").unwrap();
                }
            }
            println!();
        }
    } 
}
