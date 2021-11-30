use crate::builtins::{calc::calc, cat::cat, cd::cd, echo::echo, help::help, ls::ls};
use crate::shared_functions::lex_tokenized_input;
use crate::ShellState;
use sflib::ensure_directory;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub enum Redirection {
    Overwrite,
    Append,
    NoOp,
}

/// This struct is used to construct a shellcommand,
/// be it a builtin or external command.
/// The `name` String holds the actual command name, like `echo` or `cargo`.
/// The `args` vector hold all arguments. This includes the pipe char,
/// which is later used to detect and construct a pipe.
#[derive(Debug, Clone)]
pub struct ShellCommand {
    pub name: String,
    pub args: Vec<String>,
    pub redirection: Redirection,
}

pub fn return_shellcommand(name: String, args: Vec<String>, redirection: Redirection) -> ShellCommand {
    ShellCommand {
        name,
        args,
        redirection,
    }
}

/// Implement `Display` for `ShellCommand` which will in turn also implement `.to_string()`.
impl std::fmt::Display for ShellCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.args.join(" "))
    }
}

impl ShellCommand {
    /// Constructs a new `ShellCommand` and returns it.
    /// Takes the input given by the user, unprocessed.
    pub fn new(input: &str) -> ShellCommand {
        fn get_redirection_type(input: &str) -> Redirection {
            if input.contains(">>") {
                Redirection::Append
            } else if input.contains('>') {
                Redirection::Overwrite
            } else {
                Redirection::NoOp
            }
        }
        let lexed_vec = lex_tokenized_input(input);
        ShellCommand {
            name: lexed_vec[0].clone(),
            args: lexed_vec[1..].to_vec(),
            redirection: get_redirection_type(input),
        }
    }

    /// Takes a `ShellCommand`, figures out what to do given the name,
    /// then executes it.
    /// All builtins have to be listed here and point to their given function.
    /// It is prefered that they return a string, which gets printed here,
    /// and not by the actual function, to make testing easier.
    pub fn run(shell_state: &mut ShellState, command: ShellCommand) {
        // check for piping first, because otherwise redirecting builtins
        // would match the builtin and piping
        if command.args.contains(&String::from("|"))
            || command.args.contains(&String::from(">>"))
            || command.args.contains(&String::from(">"))
        {
            println!("{}", piped_cmd(&PipedShellCommand::from(&command)));
        } else {
            match command.name.as_str() {
                "calc" => println!("{}", calc(&command.args)),
                "cat" => println!("{}", cat(&command.args)),
                "cd" => cd(shell_state, &command),
                "echo" => println!("{}", echo(&command.args)),
                "help" => help(&command.args),
                "ls" => print!("{}", ls(command.args)),
                "pwd" => println!("{}", std::env::current_dir().unwrap().display()),
                _ => {
                    print!("{}", cmd(&command));
                }
            }
        }
    }
}

/// This struct is a vector, containing all commands and their arguments
/// in a pipeline. Every command is represented by a `ShellCommand`.
#[derive(Debug)]
pub struct PipedShellCommand {
    pub commands: Vec<ShellCommand>,
}

impl PipedShellCommand {
    /// Constructs a `PipedShellCommand` from a given `ShellCommand`.
    /// Takes a `ShellCommand` containing a pipe.
    pub fn from(input: &ShellCommand) -> PipedShellCommand {
        fn get_redirection_type(input: &ShellCommand) -> Redirection {
            if input.args.contains(&String::from(">>")) {
                Redirection::Append
            } else if input.args.contains(&String::from(">")) {
                Redirection::Overwrite
            } else {
                Redirection::NoOp
            }
        }
        let parts = input.args.split(|arg| {
            arg == &String::from("|")  ||
            // Check for appending first because `>` would match both.
            arg == &String::from(">>") ||
            arg == &String::from(">")
        });
        let mut commands: Vec<ShellCommand> = Vec::new();
        for (idx, part) in parts.enumerate() {
            if idx == 0 {
                let command = ShellCommand {
                    name: input.name.clone(),
                    args: part[0..].to_vec(),
                    redirection: get_redirection_type(input),
                };
                commands.push(command);
            } else {
                let command = ShellCommand {
                    name: part[0].clone(),
                    args: part[1..].to_vec(),
                    redirection: get_redirection_type(input),
                };
                commands.push(command);
            }
        }
        PipedShellCommand { commands }
    }
}

/// Helper function to a command, optionally with args.
pub fn cmd(command: &ShellCommand) -> String {
    let mut output = String::new();
    let child = Command::new(&command.name)
        .args(&command.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();
    if let Ok(..) = child {
        child.unwrap().stdout.unwrap().read_to_string(&mut output).unwrap();
    } else {
        println!("Sorry, '{}' was not found!", command.name);
    }
    output
}

/// This is a function for checking if the command is piped.
/// Used to remove a lot of duplicate code.
pub fn is_piped(args: &[String], cmd: &str) {
    fn run_pipe(cmd: &str, args: &[String], redirection: Redirection) {
        let command = return_shellcommand(cmd.to_string(), args.to_vec(), redirection);
        let pipe = PipedShellCommand::from(&command);
        piped_cmd(&pipe);
    }
    if args.contains(&"|".to_string()) {
        run_pipe(cmd, args, Redirection::NoOp);
    } else if args.contains(&">>".to_string()) {
        run_pipe(cmd, args, Redirection::Append);
    } else if args.contains(&">".to_string()) {
        run_pipe(cmd, args, Redirection::Overwrite);
    } else {
        // Do nothing.
    }
}

/// Returns a `Child` of a command wrapped in a `Result`.
fn return_child(cmd: &str, args: &[String]) -> Result<std::process::Child, ()> {
    Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .or(Err(()))
}

/// Takes a `PipedShellCommand`, iterating over all `ShellCommand` structs
/// contained by it, checking if it is the first or the last in the pipeline,
/// and taking the appropriate meassurements to pipe stdout.
pub fn piped_cmd(pipe: &PipedShellCommand) -> String {
    let mut output_prev = String::new();
    match pipe.commands[0].name.as_str() {
        "cat" => output_prev = cat(&pipe.commands[0].args.clone()),
        "echo" => output_prev = echo(&pipe.commands[0].args.clone()),
        "calc" => output_prev = calc(&pipe.commands[0].args.clone()),
        "ls" => output_prev = ls(pipe.commands[0].args.clone()),
        _ => {
            let child = return_child(&pipe.commands[0].name.clone(), &pipe.commands[0].args);
            if child.is_err() {
                println!("{} failed", pipe.commands[0].name.clone());
            }
            child.unwrap().stdout.unwrap().read_to_string(&mut output_prev).unwrap();
        }
    }
    for (idx, command) in pipe.commands.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        if idx == pipe.commands.len() - 1 {
            break;
        }
        match command.name.as_str() {
            "cat" => output_prev = cat(&command.args.clone()),
            "echo" => output_prev = echo(&command.args.clone()),
            "calc" => output_prev = calc(&command.args.clone()),
            "ls" => output_prev = ls(command.args.clone()),
            _ => {
                let child = return_child(&command.name.clone(), &command.args);
                match child {
                    Ok(mut child) => {
                        child.stdin.take().unwrap().write_all(output_prev.as_bytes()).unwrap();
                        output_prev = "".to_string();
                        child.stdout.unwrap().read_to_string(&mut output_prev).unwrap();
                    }
                    Err(_) => println!("{} failed", command.name.clone()),
                }
            }
        }
    }
    let file_string = &pipe.commands[pipe.commands.len() - 1].name;
    if file_string.contains('/') {
        let file_vec: Vec<&str> = file_string.split('/').collect();
        let mut parent_dir = String::new();
        for (id, chunk) in file_vec.iter().enumerate() {
            if id == file_vec.len() - 1 {
                break;
            }
            let part = format!("{}/", chunk);
            parent_dir.push_str(&part);
        }
        ensure_directory(&parent_dir, true).unwrap();
    }
    let file_path = &Path::new(file_string);
    match pipe.commands[pipe.commands.len() - 1].redirection {
        Redirection::Overwrite => {
            let mut file = std::fs::File::create(file_path).unwrap();
            file.write_all(output_prev.as_bytes()).unwrap();
            return String::new();
        }
        Redirection::Append => {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(file_path)
                .unwrap();
            writeln!(file, "{}", output_prev).unwrap();
            return String::new();
        }
        Redirection::NoOp => (),
    }
    match pipe.commands[pipe.commands.len() - 1].name.as_str() {
        "cat" => cat(&pipe.commands[pipe.commands.len() - 1].args.clone()),
        "echo" => echo(&pipe.commands[pipe.commands.len() - 1].args.clone()),
        "calc" => calc(&pipe.commands[pipe.commands.len() - 1].args.clone()),
        "ls" => ls(pipe.commands[pipe.commands.len() - 1].args.clone()),
        _ => {
            let child = return_child(
                &pipe.commands[pipe.commands.len() - 1].name.clone(),
                &pipe.commands[pipe.commands.len() - 1].args,
            );
            match child {
                Ok(mut child) => {
                    child.stdin.take().unwrap().write_all(output_prev.as_bytes()).unwrap();
                    let mut output = String::new();
                    match child.stdout.take().unwrap().read_to_string(&mut output) {
                        Err(why) => return format!("ERROR: could not read cmd2 stdout: {}", why),
                        Ok(_) => output,
                    }
                }
                Err(_) => pipe.commands[pipe.commands.len() - 1].name.clone(),
            }
        }
    }
}
