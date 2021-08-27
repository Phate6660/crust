#[cfg(feature = "readline")]
use rustyline::{Editor, Result};

mod calc;
mod cd;
mod ls;

use cd::cd;
use calc::calc_run;
use ls::ls;

#[cfg(not(feature = "readline"))]
use std::io::Write;

use std::process::Command;
use std::io::Read;

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

fn piped_cmd(input: &str) {
    let input = input.split('|').collect::<Vec<&str>>();
    let mut cmd1 = input[0].split(' ').collect::<Vec<&str>>();
    let mut cmd2 = input[1].split(' ').collect::<Vec<&str>>();
    cmd1.pop();
    cmd2.remove(0);
    let child1 = Command::new(cmd1[0])
        .args(&cmd1[1..])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .output()
        .or(Err(()));
    if child1.is_err() {
        println!("Sorrry, '{}' was not found!", input[0]);
    } else {
        let child2 = match Command::new(cmd2[0])
            .args(&cmd2[1..])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn() {
                Err(why) => panic!("couldn't spwn cmd2: {}", why),
                Ok(child2) => child2,
            };
        match child2.stdin.unwrap().write_all(String::from_utf8_lossy(&child1.unwrap().stdout).trim().as_bytes()) {
            Err(why) => println!("ERROR: couldn't write cmd2 stdin because of {}", why),
            Ok(_) => (),
        }
        let mut output = String::new();
        match child2.stdout.unwrap().read_to_string(&mut output) {
            Err(why) => println!("could not read cmd2 stdout: {}", why),
            Ok(_) => println!("{}", output.trim()),
        }
    }
}

fn piped_text(input: &str, args: bool, cmd: Vec<&str>) {
    if args {
        let child = match Command::new(cmd[0])
            .args(&cmd[1..])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn() {
                Err(why) => panic!("couldn't spwn cmd: {}", why),
                Ok(child) => child,
            };
        match child.stdin.unwrap().write_all(input.as_bytes()) {
            Err(why) => println!("ERROR: couldn't write cmd stdin because of {}", why),
            Ok(_) => (),
        }
        let mut output = String::new();
        match child.stdout.unwrap().read_to_string(&mut output) {
            Err(why) => println!("could not read cmd stdout: {}", why),
            Ok(_) => println!("{}", output.trim()),
        }
    } else {
        let child = match Command::new(cmd[0])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn() {
                Err(why) => panic!("couldn't spwn cmd: {}", why),
                Ok(child) => child,
            };
        match child.stdin.unwrap().write_all(input.as_bytes()) {
            Err(why) => println!("ERROR: couldn't write cmd stdin because of {}", why),
            Ok(_) => (),
        }
        let mut output = String::new();
        match child.stdout.unwrap().read_to_string(&mut output) {
            Err(why) => println!("could not read cmd stdout: {}", why),
            Ok(_) => println!("{}", output.trim()),
        }
    }
}

fn cmd(input: &str, args: bool) {
    if args {
        let input = input.split(' ').collect::<Vec<&str>>();
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

fn run_command(input: String) {
    if input.starts_with("calc") {
        calc_run(&input);
    } else if input.starts_with("cd") {
        fn cd_helper(dir: &str) {
            if cd(dir).is_err() {
                println!("Failed to change directory to '{}'", dir);
            }
        }
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
            std::process::exit(input.parse::<i32>().unwrap_or(0));
        } else {
            std::process::exit(0);
        }
    } else if input.starts_with("help") {
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
        ")
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
