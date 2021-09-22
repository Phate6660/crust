extern crate term;
use crate::piped_text;
use crate::shared_functions::{ShellState, ShellCommand, PipedShellCommand, get_calc_vars};
use std::io::prelude::*;

fn calc_run(problem: &str) {
    let (math_op, first_number, second_number) = get_calc_vars(problem);
    match math_op {
        "x" => println!("{}", first_number * second_number),
        "/" => println!("{}", first_number / second_number),
        "+" => println!("{}", first_number + second_number),
        "-" => println!("{}", first_number - second_number),
        _ => println!("Error, '{}' is an unsupported operation.", math_op),
    }
}

fn calc_return(problem: &str) -> i32 {
    let (math_op, first_number, second_number) = get_calc_vars(problem);
    match math_op {
        "x" => first_number * second_number,
        "/" => first_number / second_number,
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        _ => 123456789,
    }
}

pub fn calc(command: ShellCommand) {
    let problem = command.args.concat();
    /*if command.args.contains('|') {
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
    } else {*/

    calc_run(&problem);
}

fn cd_helper(dir: &str) {
    let path = std::path::Path::new(dir);
    match std::env::set_current_dir(&path) {
        Ok(()) => (),
        Err(_) => println!("Failed to change directory to '{}'", path.display()),
    }
}

pub fn cd(shell_state: &mut ShellState, input: &str) {
    if input == "cd" {
        shell_state.cd_prev_dir = Some(std::env::current_dir().unwrap().to_owned());
        let user = std::env::var("USER").unwrap();
        let home = ["/home/", user.as_str()].concat();
        cd_helper(&home);
    } else if input == "cd -" {
        if shell_state.cd_prev_dir.is_none() {
            println!("No previous dir found");
            return
        }
        // unwrap can be safely used here, because function would've returned
        // if cd_prev_dir is None
        match &shell_state.cd_prev_dir.as_ref().unwrap().to_str() {
            Some(path) => cd_helper(path),
            None => {
                println!("Could not convert Path to String (src/buildins.rs in function cd)");
                shell_state.cd_prev_dir = None;
            },
        }
        shell_state.cd_prev_dir = Some(std::env::current_dir().unwrap().to_owned());
    } else {
        shell_state.cd_prev_dir = Some(std::env::current_dir().unwrap().to_owned());
        let input = input.split(' ').collect::<Vec<&str>>()[1];
        cd_helper(input);
    }
}

pub fn echo(input: &str) {
    /*if input.contains('|') {
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
    } else {*/
        let input: Vec<&str> = input.split(' ').collect();
        let output = &input[1..];
        for arg in output {
            print!("{} ", arg);
            std::io::stdout().flush().unwrap();
        }
        println!();
    //}
}

fn ls_do(input: &str) {
    let mut t = term::stdout().unwrap();

    let path;
    if std::path::Path::new(input).exists() {
        path = std::fs::read_dir(input).unwrap()
    } else {
        println!("ERROR: '{}' is not a valid file or directory.", input);
        return;
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

pub fn ls(input: &str) {
    if input == "ls" {
        ls_do(".");
    }/* else if input.contains('|') {
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
        }*/
    else {
        let input = input.split(' ').collect::<Vec<&str>>()[1];
        ls_do(input);
    }
}

pub fn help() {
    println!(
        "\
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
    "
    );
}

