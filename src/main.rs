#[cfg(feature = "readline")]
use rustyline::Result;

mod calc;
mod prompt;

use calc::calc_run;
use prompt::display;
use std::process::Command;

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

fn run_command(input: String) {
    if input.starts_with("calc") {
        calc_run(&input);
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

#[cfg(feature = "readline")]
fn main() -> Result<()> {
    let crusty_prompt = String::from("[crusty]: ");
    let args = std::env::args().collect::<Vec<String>>();
    let na = String::from("no args");
    if args.get(1).unwrap_or(&na) == "-c" {
        let input = parse_input("non-interactive");
        run_command(input);
        std::process::exit(0);
    }
    loop {
        display(crusty_prompt.clone())?;
        let input = parse_input("interactive");
        run_command(input);
    }
}

#[cfg(not(feature = "readline"))]
fn main() {
    let crusty_prompt = String::from("[crusty]: ");
    let args = std::env::args().collect::<Vec<String>>();
    let na = String::from("no args");
    if args.get(1).unwrap_or(&na) == "-c" {
        let input = parse_input("non-interactive");
        run_command(input);
        std::process::exit(0);
    }
    loop {
        display(crusty_prompt.clone());
        let input = parse_input("interactive");
        run_command(input);
    }
}
