use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

/// Helper function to a command, optionally with args.
pub fn cmd(input: &str, args: bool) {
    if args {
        let input: Vec<&str> = input.split(' ').collect();
        let child = Command::new(&input[0])
            .args(&input[1..])
            .spawn()
            .or(Err(()));
        if child.is_err() {
            println!("Sorry, '{}' was not found!", input[0]);
        } else {
            child.unwrap().wait().unwrap();
        }
    } else {
        let child = Command::new(&input).spawn().or(Err(()));
        if child.is_err() {
            println!("Sorry, '{}' was not found!", input);
        } else {
            child.unwrap().wait().unwrap();
        }
    }
}
/// Ensures that a directory exists.
fn ensure_directory(dir: &Path) {
    if !dir.exists() {
        std::fs::create_dir(dir).unwrap();
    }
}

/// Get the calculator vars (math_op, first_number, second_number) for calc.
pub fn get_calc_vars(problem: &str) -> (&str, i32, i32) {
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
    let problem_vector: Vec<&str> = problem.split(math_op).collect();
    let first_number: i32 = problem_vector[0].parse().unwrap();
    let second_number: i32 = problem_vector[1].parse().unwrap();
    (math_op, first_number, second_number)
}

/// Get the vars required in the main function and/or the rest of the program.
pub fn main_vars() -> (Vec<String>, String, String, String) {
    let args = std::env::args().collect::<Vec<String>>();
    let crusty_prompt = std::env::var("PROMPT").unwrap_or_else(|_| String::from("[crusty]: "));
    let na = String::from("no args");
    let user = std::env::var("USER").unwrap();
    let home = ["/home/", user.as_str()].concat();
    let share_dir = [home.as_str(), "/.local/share/crusty"].concat();
    ensure_directory(Path::new(&share_dir));
    (args, crusty_prompt, na, share_dir)
}

/// A helper function to run a non-interactive command,
/// it will automatically check if `-c` was passed as an arg
/// and run commands non-interactively.
pub fn non_interactive(args: Vec<String>, na: String) {
    if args.get(1).unwrap_or(&na) == "-c" {
        let input = parse_input("non-interactive");
        crate::process_input(input);
        std::process::exit(0);
    }
}

/// A function to parse input, used for the barebones prompt.
pub fn parse_input(op: &str) -> String {
    if op == "interactive" {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read user input");
        input.trim().to_string()
    } else {
        std::env::args()
            .collect::<Vec<String>>()
            .get(2)
            .unwrap()
            .replace('"', "")
            .trim()
            .to_string()
    }
}

/// A function to pipe the output of one command into another.
pub fn piped_cmd(input: &str) {
    let input: Vec<&str> = input.split('|').collect();
    let mut cmd1: Vec<&str> = input[0].split(' ').collect();
    let mut cmd2: Vec<&str> = input[1].split(' ').collect();
    cmd1.pop();
    cmd2.remove(0);
    let child1 = Command::new(cmd1[0])
        .args(&cmd1[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .output()
        .or(Err(()));
    if child1.is_err() {
        println!("Sorrry, '{}' was not found!", input[0]);
    } else {
        let child2 = match Command::new(cmd2[0])
            .args(&cmd2[1..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("ERROR: couldn't spwn cmd2: {}", why),
            Ok(child2) => child2,
        };
        if let Err(why) = child2.stdin.unwrap().write_all(
            String::from_utf8_lossy(&child1.unwrap().stdout)
                .trim()
                .as_bytes(),
        ) {
            println!("ERROR: couldn't write to cmd2's stdin because of {}", why)
        }
        let mut output = String::new();
        match child2.stdout.unwrap().read_to_string(&mut output) {
            Err(why) => println!("ERROR: could not read cmd2 stdout: {}", why),
            Ok(_) => println!("{}", output.trim()),
        }
    }
}

/// A function to pipe text into a command.
pub fn piped_text(input: &str, args: bool, cmd: Vec<&str>) {
    if args {
        let child = match Command::new(cmd[0])
            .args(&cmd[1..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("couldn't spwn cmd: {}", why),
            Ok(child) => child,
        };
        if let Err(why) = child.stdin.unwrap().write_all(input.as_bytes()) {
            println!("ERROR: couldn't write cmd stdin because of {}", why)
        }
        let mut output = String::new();
        match child.stdout.unwrap().read_to_string(&mut output) {
            Err(why) => println!("could not read cmd stdout: {}", why),
            Ok(_) => println!("{}", output.trim()),
        }
    } else {
        let child = match Command::new(cmd[0])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("couldn't spwn cmd: {}", why),
            Ok(child) => child,
        };
        if let Err(why) = child.stdin.unwrap().write_all(input.as_bytes()) {
            println!("ERROR: couldn't write cmd stdin because of {}", why)
        }
        let mut output = String::new();
        match child.stdout.unwrap().read_to_string(&mut output) {
            Err(why) => println!("could not read cmd stdout: {}", why),
            Ok(_) => println!("{}", output.trim()),
        }
    }
}
