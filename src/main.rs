#[cfg(feature = "readline")]
use rustyline::{Editor, error::ReadlineError};

use std::io::Write;

mod builtins;
mod shared_functions;

use builtins::{calc_return, calc_run, help, ls};
use shared_functions::{
    cd_helper, cmd, 
    main_vars,
    non_interactive, 
    piped_cmd, piped_text
};

#[cfg(not(feature = "readline"))]
use shared_functions::parse_input;

use std::process::exit;

fn run_command(input: String) {
    if input.starts_with("calc") {
        let problem = input.split(' ').collect::<Vec<&str>>()[1].trim();
        if input.contains('|') {
            let calculation = calc_return(problem);
            let line_vector: Vec<&str> = input.split('|').collect();
            let cmd2 = line_vector[1];
            let mut cmd2_with_args: Vec<&str> = cmd2.split(' ').collect();
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
            // Default to /home directory in case $HOME isn't set for some reason.
            let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
            cd_helper(&home);
        } else {
            let input = input.split(' ').collect::<Vec<&str>>()[1];
            cd_helper(input);
        }
    } else if input.starts_with("echo") {
        if input.contains('|') {
            let line_vector: Vec<&str> = input.split('|').collect();
            let cmd = line_vector[0];
            let mut cmd_vector: Vec<&str> = cmd.split(' ').collect();
            cmd_vector.remove(0);
            let mut message = "".to_string();
            for word in cmd_vector {
                message.push_str(word);
            }
            let cmd2 = line_vector[1];
            let mut cmd2_with_args: Vec<&str> = cmd2.split(' ').collect();
            cmd2_with_args.remove(0);
            if cmd2.contains(' ') {
                piped_text(&message, true, cmd2_with_args);
            } else {
                piped_text(&message, false, cmd2_with_args);
            }
        } else {
            let input: Vec<&str> = input.split(' ').collect();
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
                let mut cmd_with_args: Vec<&str> = cmd.split(' ').collect();
                cmd_with_args.remove(0);
                piped_text(&output, true, cmd_with_args);
            } else {
                let cmd: Vec<&str> = cmd.split(' ').collect();
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

#[cfg(feature = "readline")]
fn main() {
    let (args, crusty_prompt, na) = main_vars();
    if args.get(1).unwrap_or(&na) == "-c" {
        non_interactive();
    }
    let mut rl = Editor::<()>::new();
    loop {
        let prompt = rl.readline(&crusty_prompt);
        match prompt {
            Ok(line) => {
                run_command(line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}

#[cfg(not(feature = "readline"))]
fn main() {
    let (args, crusty_prompt, na) = main_vars();
    if args.get(1).unwrap_or(&na) == "-c" {
        non_interactive();
    }
    loop {
        print!("{}", crusty_prompt);
        std::io::stdout().flush().unwrap();
        let input = parse_input("interactive");
        run_command(input);
    }
}
