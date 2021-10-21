use crate::builtins::{calc::calc, cd::cd, echo::echo, help::help, ls::ls};
use std::env::var as env_var;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Holds all important informations for and about the shell.
pub struct ShellState {
    pub args: Vec<String>,
    pub prompt: String,
    pub user: String,
    pub home: String,
    pub na: String,
    pub share_dir: String,
    pub cd_prev_dir: Option<PathBuf>
}

/// Ensures that a directory exists.
fn ensure_directory(dir: &Path) {
    if !dir.exists() {
        std::fs::create_dir(dir).unwrap();
    }
}

impl ShellState {
    /// Initalizes the shell state with all the informations needed.
    ///
    /// cd_prev_dir doesnt hold a value, because there is no previous dir yet.
    pub fn init() -> ShellState {
        let args = std::env::args().collect();
        let prompt = env_var("PROMPT").unwrap_or_else(|_| String::from("[crusty]: "));
        let user_command = return_shellcommand(
            String::from("whoami"),
            Vec::new(),
            Redirection::NoOp
        );
        let user = env_var("USER").unwrap_or_else(|_| cmd(user_command));
        let home = env_var("HOME").unwrap_or_else(|_| ["/home/", user.as_str()].concat());
        let na = String::from("no args");
        let share_dir = [&home, "/.local/share/crusty"].concat();
        let cd_prev_dir = None;
        let shell_state = ShellState { args, prompt, user, home, na, share_dir, cd_prev_dir };
        ensure_directory(Path::new(&shell_state.share_dir));
        shell_state
    }
    pub fn eval_prompt(prompt_string: &str) -> String {
        let split_prompt: Vec<&str> = prompt_string.split("%E").collect();
        let mut commands: Vec<ShellCommand> = Vec::new();
        let mut maybe_sep: bool = false;
        let mut start_record: bool = false;
        let mut command: String = String::new();
        for c in prompt_string.chars() {
            if !start_record && c == '%' {
                maybe_sep = true;
                continue;
            }
            if !start_record && maybe_sep && c == 'E' {
                maybe_sep = false;
                start_record = true;
                continue;
            }
            if start_record {
                command.push(c);
            }
            if start_record && c == '%' {
                maybe_sep = true;
                continue;
            }
            if maybe_sep && c == 'E' {
                maybe_sep = false;
                start_record = false;
                command.pop();
                command.pop();
                commands.push(ShellCommand::new(command.clone()));
                command = String::from("");
                continue;
            }
        }
        let mut evaled_prompt: String = String::new();
        for (idx, split) in split_prompt.iter().enumerate() {
            if idx % 2 == 0 {
                evaled_prompt.push_str(split);
                continue;
            }
            // The part in the index maps the index of the split_prompt
            // vector to the right command in the commands vector.
            //
            // Every second index of the split_prompt vector is a command to be executed.
            //
            // So the value at index 1 of the split_prompt is
            // the appropriate command at index 0 of the commands vector.
            //
            // We add 1 to the index, because otherwise the division
            // wouldn't return a valid interger, from which 1 can be
            // subtracted to get the according index in the commands vector.
            #[rustfmt::skip]
            let command_output = cmd(
                commands[
                    if idx == 1 {
                        0
                    } else {
                        ((idx + 1) / 2) - 1
                    }
                ].clone()
            );
            evaled_prompt.push_str(command_output.as_str());
            evaled_prompt.pop();
        }
        evaled_prompt
    }
}

#[derive(Debug, Clone)]
pub enum Redirection {
    Overwrite,
    Append,
    NoOp
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
    pub redirection: Redirection
}

pub fn return_shellcommand(name: String, args: Vec<String>, redirection: Redirection) -> ShellCommand {
    ShellCommand {
        name,
        args,
        redirection
    }
}

/// Tokenizes the input, returning a vector of every character in `input`.
fn tokenize(input: &str) -> Vec<String> {
    let mut tokenized_vec: Vec<&str> = input.split("").collect();
    // The first and last elements are blank and need to be removed.
    tokenized_vec.remove(0);
    tokenized_vec.remove(tokenized_vec.len() - 1);
    tokenized_vec.iter().map(|t| t.to_string()).collect()
}

/// Creates a lexified vector from a tokenized one.
/// Example, if the tokenized vec was:
/// ```
/// ["e", "c", "h", "o",
///  " ",
///  "\"", "a", "r", "g", " ", "1" "\"",
///  " ",
///  "\"", "a", "r", "g", " ", "2" "\""]
/// ```
/// It would return:
/// `["echo", "arg 1", "arg 2"]`
fn lex_tokenized_input(tokenized_vec: &[String]) -> Vec<String> {
    fn push_to_vec(from_vec: &mut Vec<String>, to_vec: &mut Vec<String>) {
        let element = from_vec.concat();
        // Don't push to the vector if element is empty.
        if element.is_empty() {
            return;
        }
        to_vec.push(element);
        from_vec.clear();
    }
    // This is the final vector that will be returned.
    let mut lexed_vec: Vec<String> = Vec::new();
    // This is a temporary vec that gets pushed to lexed_vec.
    let mut tmp_vec: Vec<String> = Vec::new();
    // Same as tmp_vec except this is for anything in quotes.
    let mut quoted_vec: Vec<String> = Vec::new();
    // These two bools are used for checking if the character is in quotes,
    // and if the quotes part of the match statement was ran.
    let mut quoted = false;
    let mut quotes_ran = false;
    for (idx, character) in tokenized_vec.iter().enumerate() {
        match character.as_str() {
            // TODO: Figure out a more efficient way for this.
            // Ranges only work with chars and numbers.
            "a" | "b" | "c" | "d" | "e" | 
            "f" | "g" | "h" | "i" | "j" | 
            "k" | "l" | "m" | "n" | "o" | 
            "p" | "q" | "r" | "s" | "t" | 
            "u" | "v" | "w" | "x" | "y" | 
            "z" | "0" | "1" | "2" | "3" |
            "4" | "5" | "6" | "7" | "8" | 
            "9" | "." | "/" | "(" | ")" |
            ">" | "|" => {
                if quoted {
                    quoted_vec.push(character.to_string());
                } else {
                    tmp_vec.push(character.to_string());
                    // Needed to push the last element to lexed_vec.
                    if idx == tokenized_vec.len() - 1 {
                        push_to_vec(&mut tmp_vec, &mut lexed_vec);
                    }
                }
            },
            "\"" | "'" => {
                if quotes_ran {
                    push_to_vec(&mut quoted_vec, &mut lexed_vec);
                    quoted = false;
                    quotes_ran = false;
                } else {
                    quoted = true;
                    quotes_ran = true;
                }
            },
            " " => { 
                if quoted {
                    quoted_vec.push(character.to_string());
                } else {
                    push_to_vec(&mut tmp_vec, &mut lexed_vec);
                }
            },
            _ => println!("'{}' is an unsupported character.", character),
        }
    }
    lexed_vec
}

impl ShellCommand {
    /// Constructs a new ShellCommand and returns it.
    /// Takes the input given by the user, unprocessed.
    pub fn new(input: String) -> ShellCommand {
        fn get_redirection_type(input: &str) -> Redirection {
            if input.contains(">>") {
                Redirection::Append
            } else if input.contains('>') {
                Redirection::Overwrite
            } else {
                Redirection::NoOp
            }
        }
        let tokenized_vec = tokenize(&input);
        let lexed_vec = lex_tokenized_input(&tokenized_vec);
        ShellCommand {
            name: lexed_vec[0].clone(),
            args: lexed_vec[1..].to_vec(),
            redirection: get_redirection_type(&input)
        }
    }
    /// Takes a ShellCommand, figures out what to do given the name,
    /// then executes it.
    /// All builtins have to be listed here and point to their given function.
    /// It is prefered that they return a string, which gets printed here,
    /// and not by the actual function, to make testing easier.
    pub fn run(shell_state: &mut ShellState, command: ShellCommand) {
        match command.name.as_str() {
            "calc" => println!("{}", calc(command.args)),
            "cd" => cd(shell_state, command),
            "echo" => println!("{}", echo(command.args)),
            "help" => help(command.args),
            "ls" => print!("{}", ls(command.args)),
            "pwd" => println!("{}", std::env::current_dir().unwrap().display()),
            _ => {
                if command.args.contains(&String::from("|"))
                    || command.args.contains(&String::from(">>"))
                    || command.args.contains(&String::from(">"))
                {
                    piped_cmd(PipedShellCommand::from(command));
                } else {
                    print!("{}", cmd(command));
                }
            }
        }
    }
}

/// This struct is a vector, containing all commands and their arguments
/// in a pipeline. Every command is represented by a ShellCommand.
#[derive(Debug)]
pub struct PipedShellCommand {
    pub commands: Vec<ShellCommand>
}

impl PipedShellCommand {
    /// Constructs a PipedShellCommand from a given ShellCommand.
    /// Takes a ShellCommand containing a pipe.
    pub fn from(input: ShellCommand) -> PipedShellCommand {
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
                    redirection: get_redirection_type(&input)
                };
                commands.push(command);
            } else {
                let command = ShellCommand {
                    name: part[0].clone(),
                    args: part[1..].to_vec(),
                    redirection: get_redirection_type(&input)
                };
                commands.push(command);
            }
        }
        PipedShellCommand { commands }
    }
}

/// Helper function to a command, optionally with args.
pub fn cmd(command: ShellCommand) -> String {
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
        std::io::stdin().read_line(&mut input).expect("failed to read user input");
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

/// Takes a PipedShellCommand, iterating over all ShellCommand structs
/// contained by it, checking if it is the first or the last in the pipeline,
/// and taking the appropriate meassurements to pipe stdout.
pub fn piped_cmd(pipe: PipedShellCommand) {
    let mut output_prev = String::new();
    match pipe.commands[0].name.as_str() {
        "echo" => output_prev = echo(pipe.commands[0].args.clone()),
        "calc" => output_prev = calc(pipe.commands[0].args.clone()),
        "ls" => output_prev = ls(pipe.commands[0].args.clone()),
        _ => {
            let child = Command::new(pipe.commands[0].name.clone())
                .args(&pipe.commands[0].args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .or(Err(()));
            if child.is_err() {
                println!("{} failed", pipe.commands[0].name.clone());
            }
            child.unwrap().stdout.unwrap().read_to_string(&mut output_prev).unwrap();
        }
    }
    for (idx, command) in pipe.commands.iter().enumerate() {
        if idx == 0 {
            continue;
        } else if idx == pipe.commands.len() - 1 {
            break;
        } else {
            match command.name.as_str() {
                "echo" => output_prev = echo(command.args.clone()),
                "calc" => output_prev = calc(command.args.clone()),
                "ls" => output_prev = ls(command.args.clone()),
                _ => {
                    let child = Command::new(command.name.clone())
                        .args(&command.args)
                        .stdout(Stdio::piped())
                        .stdin(Stdio::piped())
                        .spawn()
                        .or(Err(()));
                    match child {
                        Ok(mut child) => {
                            child.stdin.take().unwrap().write_all(output_prev.as_bytes()).unwrap();
                            output_prev = "".to_string();
                            child.stdout.unwrap().read_to_string(&mut output_prev).unwrap();
                        },
                        Err(_) => println!("{} failed", command.name.clone())
                    }
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
        ensure_directory(Path::new(&parent_dir));
    }
    let file_path = &Path::new(file_string);
    match pipe.commands[pipe.commands.len() - 1].redirection {
        Redirection::Overwrite => {
            let mut file = std::fs::File::create(file_path).unwrap();
            file.write_all(output_prev.as_bytes()).unwrap();
        },
        Redirection::Append => {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(file_path)
                .unwrap();
            writeln!(file, "{}", output_prev).unwrap();
        },
        Redirection::NoOp => ()
    }
    match pipe.commands[pipe.commands.len() - 1].name.as_str() {
        "echo" => print!("{}", echo(pipe.commands[pipe.commands.len() - 1].args.clone())),
        "calc" => print!("{}", calc(pipe.commands[pipe.commands.len() - 1].args.clone())),
        "ls" => print!("{}", ls(pipe.commands[pipe.commands.len() - 1].args.clone())),
        _ => {
            let child = Command::new(pipe.commands[pipe.commands.len() - 1].name.clone())
                .args(&pipe.commands[pipe.commands.len() - 1].args)
                .stdout(Stdio::piped())
                .stdin(Stdio::piped())
                .spawn()
                .or(Err(()));
            match child {
                Ok(mut child) => {
                    child.stdin.take().unwrap().write_all(output_prev.as_bytes()).unwrap();
                    let mut output = String::new();
                    match child.stdout.take().unwrap().read_to_string(&mut output) {
                        Err(why) => println!("ERROR: could not read cmd2 stdout: {}", why),
                        Ok(_) => println!("{}", output)
                    }
                },
                Err(_) => println!("{} failed", pipe.commands[pipe.commands.len() - 1].name.clone())
            }
        }
    }
}
