use crate::shared_functions::{
    get_calc_vars, piped_cmd, PipedShellCommand, ShellCommand, ShellState,
};
use colored::*;

/// Takes the `args` part of a ShellCommand struct and tries
/// to evaluate the given mathematical expression, returning a String with the
/// result.
pub fn calc(args: Vec<String>) -> String {
    let mut output = String::new();
    if args.contains(&"|".to_string()) {
        let command = ShellCommand {
            name: "calc".to_string(),
            args,
            redirect: false,
        };
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else if args.contains(&">".to_string()) {
        let command = ShellCommand {
            name: "calc".to_string(),
            args,
            redirect: true,
        };
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else {
        let problem = args.concat();
        let (math_op, first_number, second_number) = get_calc_vars(&problem);
        match math_op {
            "x" => output.push_str(format!("{}", first_number * second_number).as_str()),
            "/" => output.push_str(format!("{}", first_number / second_number).as_str()),
            "+" => output.push_str(format!("{}", first_number + second_number).as_str()),
            "-" => output.push_str(format!("{}", first_number - second_number).as_str()),
            _ => output.push_str(format!("Error, '{}' is an unsupported operation.", math_op).as_str()),
        }
    }
    output
}

/// Helper for cd, to actually change the dirctory.
fn cd_helper(dir: &str) {
    let path = std::path::Path::new(dir);
    match std::env::set_current_dir(&path) {
        Ok(()) => (),
        Err(_) => println!("Failed to change directory to '{}'", path.display()),
    }
}

/// Used to change directory.
/// Takes a ShellState and ShellCommand.
/// ShellState is used to realize `cd -` fuctionality, but can be used for
/// other options in the future.
pub fn cd(shell_state: &mut ShellState, command: ShellCommand) {
    if command.args.is_empty() {
        shell_state.cd_prev_dir = Some(std::env::current_dir().unwrap());
        let user = std::env::var("USER").unwrap();
        let home = ["/home/", user.as_str()].concat();
        cd_helper(&home);
    } else if command.args[0] == "-" {
        if shell_state.cd_prev_dir.is_none() {
            println!("No previous dir found");
            return;
        }
        // unwrap can be safely used here, because function would've returned
        // if cd_prev_dir is None
        match &shell_state.cd_prev_dir.as_ref().unwrap().to_str() {
            Some(path) => cd_helper(path),
            None => {
                println!("Could not convert Path to String (src/buildins.rs in function cd)");
                shell_state.cd_prev_dir = None;
            }
        }
        shell_state.cd_prev_dir = Some(std::env::current_dir().unwrap());
    } else {
        shell_state.cd_prev_dir = Some(std::env::current_dir().unwrap());
        cd_helper(&command.args[0]);
    }
}

/// Just like you know it. Takes the args part of ShellCommand and prints them.
pub fn echo(args: Vec<String>) -> String {
    let mut output = String::new();
    if args.contains(&"|".to_string()) {
        let command = ShellCommand {
            name: "echo".to_string(),
            args,
            redirect: false,
        };
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else if args.contains(&">".to_string()) {
        let command = ShellCommand {
            name: "echo".to_string(),
            args,
            redirect: true,
        };
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else {
        for arg in args {
            output.push_str(format!("{} ", arg).as_str());
        }
    }
    output
}

/// List dir entries. Take the args part of ShellCommand.
pub fn ls(mut args: Vec<String>) -> String {
    let mut output = String::new();
    if args.is_empty() {
        args.push(".".to_string());
    }

    if args.contains(&"|".to_string()) {
        let command = ShellCommand {
            name: "ls".to_string(),
            args,
            redirect: false,
        };
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else if args.contains(&">".to_string()) {
        let command = ShellCommand {
            name: "ls".to_string(),
            args,
            redirect: true,
        };
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else {
        let mut path_idx = 0;
        for (idx, arg) in args.iter().enumerate() {
            if !arg.starts_with("--") || !arg.starts_with('-') {
                path_idx = idx;
            }
        }

        let input = &args[path_idx];
        let path;
        if std::path::Path::new(input).exists() {
            path = std::fs::read_dir(input).unwrap()
        } else {
            println!("ERROR: '{}' is not a valid file or directory.", input);
            return String::from("");
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
                        output.push_str(format!("{}", part.white()).as_str());
                    } else {
                        output.push_str(format!("{}", part.green()).as_str());
                    }
                    n += 1;
                    if n == parts_count {
                        break;
                    } else {
                        output.push('/');
                    }
                }
                output.push('\n');
            }
        }
    }
    output
}

/// Prints a list of builtin commands.
pub fn help(args: Vec<String>) {
    if args.is_empty() {
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
        return;
    }
    if args.len() > 1 {
        println!("Please specify only one command.");
        return;
    }
    match args[0].as_str() {
        "calc" => {
            println!(
                "\
                Supports +, -, /, x for two numbers.\n\
                E.g. 1+1, 4/2, 2x4, 2-1\
                "
            );
        }
        "cd" => {
            println!(
                "\
                Takes a absolute or relative path and changes directory to it.\n\
                `cd -` will take you to your previous dir.\
                "
            );
        }
        "echo" => {
            println!(
                "\
                Takes n amount of arguments and prints them to stdout.\
                "
            );
        }
        "exit" => {
            println!(
                "\
                Exits the shell with the given exit code.\
                "
            );
        }
        "help" => {
            println!(
                "\
                Returns information about the builtin commands.\
                "
            );
        }
        "ls" => {
            println!(
                "\
                Lists the content of a directory.\
                "
            );
        }
        "pwd" => {
            println!(
                "\
                Prints the working directory.\
                "
            );
        }
        _ => {
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
    }
}
