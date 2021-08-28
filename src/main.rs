#[cfg(feature = "readline")]
use rustyline::{Editor, Result};

mod builtins;
mod shared_functions;

use builtins::{calc_return, calc_run, help, ls};
use shared_functions::{cd_helper, cmd, parse_input, piped_cmd, piped_text};
use std::process::exit;

#[cfg(not(feature = "readline"))]
use std::io::Write;

fn run_command(input: String) {
    if input.starts_with("calc") {
        let problem = input.split(' ').collect::<Vec<&str>>()[1].trim();
        if input.contains('|') {
            let calculation = calc_return(problem);
            let line_vector = input.split('|').collect::<Vec<&str>>();
            let cmd2 = line_vector[1];
            let mut cmd2_with_args = cmd2.split(' ').collect::<Vec<&str>>();
            cmd2_with_args.remove(0);
            if cmd2.contains(' ') {
                piped_text(&calculation.to_string(), true, cmd2_with_args);
            } else {
                piped_text(&calculation.to_string(), false, cmd2_with_args);
            }
        } else {
            calc_run(problem);
        }
    } else if input.starts_with("cd") {
        if input == "cd" {
            let home = std::env::var("HOME").unwrap();
            cd_helper(&home);
        } else {
            let input = input.split(' ').collect::<Vec<&str>>()[1];
            cd_helper(input);
        }
    } else if input.starts_with("echo") {
        if input.contains('|') {
            let line_vector = input.split('|').collect::<Vec<&str>>();
            let cmd = line_vector[0];
            let mut cmd_vector = cmd.split(' ').collect::<Vec<&str>>();
            cmd_vector.remove(0);
            let mut message = "".to_string();
            for word in cmd_vector {
                message.push_str(word);
            }
            let cmd2 = line_vector[1];
            let mut cmd2_with_args = cmd2.split(' ').collect::<Vec<&str>>();
            cmd2_with_args.remove(0);
            if cmd2.contains(' ') {
                piped_text(&message, true, cmd2_with_args);
            } else {
                piped_text(&message, false, cmd2_with_args);
            }
        } else {
            let input = input.split(' ').collect::<Vec<&str>>();
            let output = &input[1..];
            for arg in output {
                print!("{} ", arg);
                std::io::stdout().flush().unwrap();
            }
            println!();
        }
    } else if input.starts_with("exit") {
        if input.contains(' ') {
            let input = input.split(' ').collect::<Vec<&str>>()[1];
            exit(input.parse::<i32>().unwrap_or(0));
        } else {
            exit(0);
        }
    } else if input.starts_with("help") {
        help();
    } else if input.starts_with("ls") {
        if input == "ls" {
            ls(".");
        } else if input.contains('|') {
            let ls_input = input.split('|').collect::<Vec<&str>>()[0];
            let path = if ls_input.trim() == "ls" {
                std::fs::read_dir(".").unwrap()
            } else if ls_input.contains(' ') {
                let ls_input = ls_input.split(' ').collect::<Vec<&str>>()[1];
                if std::path::Path::new(ls_input).exists() {
                    std::fs::read_dir(ls_input).unwrap()
                } else {
                    std::fs::read_dir(".").unwrap()
                }
            } else {
                std::fs::read_dir(".").unwrap()
            };
            let mut output = "".to_string();
            for file in path {
                let raw_entry = file.unwrap().path();
                #[cfg(target_os = "linux")]
                let still_raw_entry = raw_entry.to_str().unwrap().replace("./", ""); 
                #[cfg(target_os = "windows")]
                let still_raw_entry = raw_entry.to_str().unwrap().replace(".\\", "");
                let paths = still_raw_entry.split('\n');
                let pre_output = "";
                for file in paths {
                    let pre_output = &[pre_output, file, "\n"].concat();
                    output.push_str(pre_output);
                }
            }
            let cmd = input.split('|').collect::<Vec<&str>>()[1];
            if cmd.contains(' ') {
                let mut cmd_with_args = cmd.split(' ').collect::<Vec<&str>>();
                cmd_with_args.remove(0);
                piped_text(&output, true, cmd_with_args);
            } else {
                let cmd = cmd.split(' ').collect::<Vec<&str>>();
                piped_text(&output, false, cmd);
            }
        } else {
            let input = input.split(' ').collect::<Vec<&str>>()[1];
            ls(input);
        }
    } else if input == "pwd" {
        println!("{}", std::env::current_dir().unwrap().display());
    } else if input.contains('|') {
        piped_cmd(&input);
    } else if input.contains(' ') {
        cmd(&input, true);
    } else {
        cmd(&input, false);
    }
}

fn vars() -> (Vec<String>, String, String) {
    let args = std::env::args().collect::<Vec<String>>();
    let crusty_prompt = String::from("[crusty]: ");
    let na = String::from("no args");
    (args, crusty_prompt, na)
}

fn non_interactive(args: Vec<String>, na: String) {
    if args.get(1).unwrap_or(&na) == "-c" {
        let input = parse_input("non-interactive");
        run_command(input);
        std::process::exit(0);
    }
}

#[cfg(feature = "readline")]
fn main() -> Result<()> {
    let (args, crusty_prompt, na) = vars();
    non_interactive(args, na);
    let mut rl = Editor::<()>::new();
    loop {
        let prompt = rl.readline(&std::env::var("PROMPT").unwrap_or_else(|_| crusty_prompt.clone()))?;
        println!("{}", prompt);
        let input = parse_input("interactive");
        run_command(input);
    }
}

#[cfg(not(feature = "readline"))]
fn main() {
    let (args, crusty_prompt, na) = vars();
    non_interactive(args, na);
    loop {
        let prompt = std::env::var("PROMPT").unwrap_or_else(|_| crusty_prompt.clone());
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        let input = parse_input("interactive");
        run_command(input);
    }
}
