extern crate term;
use crate::shared_functions::{ShellState, ShellCommand, PipedShellCommand, piped_cmd, get_calc_vars};
use std::io::prelude::*;

pub fn calc(args: Vec<String>) -> String {
    let problem = args.concat();
    let mut output = String::new();
    let (math_op, first_number, second_number) = get_calc_vars(&problem);
    match math_op {
        "x" => output.push_str(format!("{}", first_number * second_number).as_str()),
        "/" => output.push_str(format!("{}", first_number / second_number).as_str()),
        "+" => output.push_str(format!("{}", first_number + second_number).as_str()),
        "-" => output.push_str(format!("{}", first_number - second_number).as_str()),
        _ => output.push_str(format!("Error, '{}' is an unsupported operation.", math_op).as_str()),
    }
    output
}

fn cd_helper(dir: &str) {
    let path = std::path::Path::new(dir);
    match std::env::set_current_dir(&path) {
        Ok(()) => (),
        Err(_) => println!("Failed to change directory to '{}'", path.display()),
    }
}

pub fn cd(shell_state: &mut ShellState, command: ShellCommand) {
    if command.args.len() <= 0 {
        shell_state.cd_prev_dir = Some(std::env::current_dir().unwrap().to_owned());
        let user = std::env::var("USER").unwrap();
        let home = ["/home/", user.as_str()].concat();
        cd_helper(&home);
    } else if command.args[0] == "-" {
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
        cd_helper(&command.args[0]);
    }
}

pub fn echo(args: Vec<String>) -> String {
    let mut output = String::new();
    if args.contains(&"|".to_string()) {
        let command = ShellCommand {
            name: "echo".to_string(),
            args: args,
        };
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else {
        for arg in args {
            output.push_str(&format!("{} ", arg).to_string());
        }
    }
    output
}

fn ls_do(args: Vec<String>) {
    let mut t = term::stdout().unwrap();
    let mut path_idx = 0;

    for (idx, arg) in args.iter().enumerate() {
        if !arg.starts_with("--") || !arg.starts_with("-") {
            path_idx = idx;
        }
    }


    let input = &args[path_idx];
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

pub fn ls(command: ShellCommand) {
    if command.args.len() <= 0 {
        ls_do(vec![".".to_string()]);
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
        ls_do(command.args);
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
