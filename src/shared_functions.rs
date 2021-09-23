use crate::builtins::{calc, cd, echo, help, ls};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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
            share_dir: [
                ["/home/", std::env::var("USER").unwrap().as_str()]
                    .concat()
                    .as_str(),
                "/.local/share/crusty",
            ]
            .concat(),
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
        let pipe = PipedShellCommand { commands: commands };
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
pub fn piped_cmd(pipe: PipedShellCommand) {
    let child = Command::new(pipe.commands[0].name.clone())
        .args(&pipe.commands[0].args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .or(Err(()));
    if child.is_err() {
        println!("{} failed", pipe.commands[0].name.clone());
    }
    let mut output_prev = String::new();
    child
        .unwrap()
        .stdout
        .unwrap()
        .read_to_string(&mut output_prev)
        .unwrap();
    for (idx, command) in pipe.commands.iter().enumerate() {
        if idx == 0 {
            continue;
        } else if idx == pipe.commands.len() {
            break;
        } else {
            let child = Command::new(command.name.clone())
                .args(&command.args)
                .stdout(Stdio::piped())
                .stdin(Stdio::piped())
                .spawn()
                .or(Err(()));
            match child {
                Ok(mut child) => {
                    child
                        .stdin
                        .take()
                        .unwrap()
                        .write_all(output_prev.trim().as_bytes())
                        .unwrap();
                    child
                        .stdout
                        .unwrap()
                        .read_to_string(&mut output_prev)
                        .unwrap();
                }
                Err(_) => println!("{} failed", command.name.clone()),
            }
        }
    }
    let child = Command::new(pipe.commands[pipe.commands.len() - 1].name.clone())
        .args(&pipe.commands[pipe.commands.len() - 1].args)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .or(Err(()));
    match child {
        Ok(mut child) => {
            child
                .stdin
                .take()
                .unwrap()
                .write_all(output_prev.trim().as_bytes())
                .unwrap();
            let mut output = String::new();
            match child.stdout.take().unwrap().read_to_string(&mut output) {
                Err(why) => println!("ERROR: could not read cmd2 stdout: {}", why),
                Ok(_) => println!("{}", output.trim()),
            }
        }
        Err(_) => println!(
            "{} failed",
            pipe.commands[pipe.commands.len() - 1].name.clone()
        ),
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
