use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use crate::builtins::{calc, ls, help, echo, cd};

/// Holds all important informations for and about the shell
pub struct ShellState {
    pub args: Vec<String>,
    pub prompt: String,
    pub user: String,
    pub home: String,
    pub na: String,
    pub share_dir: String,
    pub cd_prev_dir: Option<PathBuf>,
}

impl ShellState {
    /// Ensures that a directory exists
    fn ensure_directory(dir: &Path) {
        if !dir.exists() {
            std::fs::create_dir(dir).unwrap();
        }
    }

    /// Initalizes the shell state with all the informations needed
    ///
    /// cd_prev_dir doesnt hold a value, because there is no previous
    /// dir yet
    pub fn init() -> ShellState {
        let shell_state = ShellState {
            args: std::env::args().collect(),
            prompt: std::env::var("PROMPT").unwrap_or_else(|_| String::from("[crusty]: ")),
            user: std::env::var("USER").unwrap(),
            home: ["/home/", std::env::var("USER").unwrap().as_str()].concat(),
            na: String::from("no args"),
            share_dir: [["/home/", std::env::var("USER").unwrap().as_str()].concat().as_str(), "/.local/share/crusty"].concat(),
            cd_prev_dir: None,
        };
        ShellState::ensure_directory(Path::new(&shell_state.share_dir));
        shell_state
    }
}

#[derive(Debug)]
pub struct ShellCommand {
    pub name: String,
    pub args: Vec<String>,
}

impl ShellCommand {
    pub fn new(input: String) -> ShellCommand {
        let split_input: Vec<&str> = input.split_whitespace().collect();
        let mut split_input_string: Vec<String> = Vec::new();
        for arg in split_input {
            split_input_string.push(arg.to_string());
        }
        let shell_command = ShellCommand {
            name: split_input_string[0].clone(),
            args: split_input_string[1..].to_vec(),
        };
        shell_command
    }
    pub fn run(shell_state: &mut ShellState, command: ShellCommand) {
        match command.name.as_str() {
            "calc" => calc(command),
            "cd" => cd(shell_state, command),
            "echo" => echo(command),
            "help" => help(),
            "ls" => ls(command),
            "pwd" => println!("{}", std::env::current_dir().unwrap().display()),
            _ => {
                if command.args.contains(&String::from("|")) {
                    piped_cmd(PipedShellCommand::from(command));
                } else {
                    cmd(command);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct PipedShellCommand {
    pub commands: Vec<ShellCommand>,
    pub text: Option<String>,
}

impl PipedShellCommand {
    pub fn from(input: ShellCommand) -> PipedShellCommand {
        let parts = input.args.split(|arg| arg == &String::from("|"));
        let mut commands: Vec<ShellCommand> = Vec::new();
        for (idx, part) in parts.enumerate() {
            if idx == 0 {
                let command = ShellCommand {
                    name: input.name.clone(),
                    args: part[0..].to_vec(),
                };
                commands.push(command);
            } else {
                let command = ShellCommand {
                    name: part[0].clone(),
                    args: part[1..].to_vec(),
                };
                commands.push(command);
            }
        }
        let pipe = PipedShellCommand {
            commands: commands,
            text: None,
        };
        pipe
    }
    pub fn with_text_from(text: String, command: ShellCommand) -> PipedShellCommand {
        let mut pipe = PipedShellCommand::from(command);
        pipe.text = Some(text);
        pipe
    }
}

/// Helper function to a command, optionally with args.
pub fn cmd(command: ShellCommand) {
    let child = Command::new(&command.name)
        .args(&command.args)
        .spawn()
        .or(Err(()));
    if child.is_err() {
        println!("Sorry, '{}' was not found!", command.name);
    } else {
        child.unwrap().wait().unwrap();
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

/// A helper function to run a non-interactive command,
/// it will automatically check if `-c` was passed as an arg
/// and run commands non-interactively.
pub fn non_interactive(shell_state: &mut ShellState) {
    if shell_state.args.get(1).unwrap_or(&shell_state.na) == "-c" {
        let input = parse_input("non-interactive");
        crate::process_input(shell_state, input);
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
