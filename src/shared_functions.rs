use crate::builtins::{calc::calc, cat::cat, cd::cd, echo::echo, help::help, ls::ls};
use std::env::var as env_var;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[cfg(feature = "readline")]
use std::process::exit;

#[cfg(feature = "readline")]
use rustyline::{error::ReadlineError, Editor};

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

/// Ensures that a directory and it's parents exist.
fn ensure_directory(dir: &Path) {
    if !dir.exists() {
        std::fs::create_dir_all(dir).unwrap();
    }
}

/// Gets the current time with the format specified if the `time` feature is enabled.
/// Otherwise it returns the format string back.
fn get_time(format: &str) -> String {
    #[cfg(not(feature = "time"))]
    {
        format.to_string()
    }
    
    #[cfg(feature = "time")]
    {
        let date = chrono::Local::now();
        date.format(format).to_string()
    }
}

// Process the input to run the appropriate builtin or external command.
fn process_input(shell_state: &mut ShellState, input: &str) {
    if input.is_empty() {
        return;
    }
    let command = ShellCommand::new(input);
    ShellCommand::run(shell_state, command);
}

#[cfg(feature = "readline")]
pub fn run_loop(prompt: &str, rl: &mut Editor<()>, history_file: &str, mut shell_state: ShellState) {
    loop {
        let prompt = rl.readline(prompt);
        match prompt {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line.starts_with("exit") {
                    if line.contains(' ') {
                        let input = line.split(' ').collect::<Vec<&str>>()[1];
                        rl.save_history(&history_file).unwrap();
                        exit(input.parse::<i32>().unwrap_or(0));
                    } else {
                        rl.save_history(&history_file).unwrap();
                        exit(0);
                    }
                }
                process_input(&mut shell_state, &line);
            },
            Err(ReadlineError::Interrupted) => {
                continue;
            },
            Err(ReadlineError::Eof) => {
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(&history_file).unwrap();
}

#[cfg(not(feature = "readline"))]
pub fn run_loop(prompt: &str, mut shell_state: ShellState) {
    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        let input = parse_input("interactive");
        process_input(&mut shell_state, &input);
    }
}

impl ShellState {
    /// Initalizes the shell state with all the informations needed.
    ///
    /// `cd_prev_dir` doesnt hold a value, because there is no previous dir yet.
    pub fn init() -> ShellState {
        let args = std::env::args().collect();
        let prompt = env_var("PROMPT").unwrap_or_else(|_| String::from("[crust]: "));
        let user_command = return_shellcommand(String::from("whoami"), Vec::new(), Redirection::NoOp);
        let user = env_var("USER").unwrap_or_else(|_| cmd(&user_command)).trim().to_string();
        let home = env_var("HOME").unwrap_or_else(|_| ["/home/", user.as_str()].concat());
        let na = String::from("no args");
        let share_dir = [&home, "/.local/share/crust"].concat();
        let cd_prev_dir = None;
        let shell_state = ShellState {args, prompt, user, home, na, share_dir, cd_prev_dir};
        ensure_directory(Path::new(&shell_state.share_dir));
        shell_state
    }
    pub fn eval_prompt(&mut self) -> String {
        let mut evaled_prompt = self.prompt.clone();
        fn prompt_lex_tokenized_input(input: &str) -> Vec<ShellCommand> {
            let tokenized_vec = tokenize(input);
            let mut tmp_vec: String = String::new();
            let mut command_vec: Vec<ShellCommand> = Vec::new();
            let mut command = false;
            let mut command_end = false;
            let mut tok_iter = tokenized_vec.iter().peekable();
            while tok_iter.peek() != None {
                let tok_iter_char = tok_iter.next().unwrap().as_str();
                if command_end {
                    command_vec.push(ShellCommand::new(tmp_vec.as_str()));
                    tmp_vec.clear();
                    command = false;
                    command_end = false;
                    continue;
                }
                if tok_iter_char == "%" && tok_iter.peek().unwrap().as_str() == "(" {
                    command = true;
                } else if command {
                    if tok_iter_char == "(" {
                        continue;
                    } else if tok_iter_char != ")" {
                        tmp_vec.push_str(tok_iter_char);
                    } else if tok_iter_char == ")" {
                        command_end = true;
                        continue;
                    }
                }
            }
            command_vec
        }
        let commands = prompt_lex_tokenized_input(&self.prompt);
        let mut command_output: String;
        for command in commands {
            if command.args.contains(&String::from("|")) {
                command_output = piped_cmd(&PipedShellCommand::from(&command));
            } else {
                command_output = cmd(&command);
            }
            evaled_prompt = evaled_prompt.replace(
                format!("%({})", command.to_string()).as_str(), command_output.trim()
            );
        }
        // TODO: Add support for more escape sequences.
        // To match an escape sequence, we need to match for an escaped version of the sequence,
        // and then replace the escaped version with the actual sequence.
        // This is because the escape sequence is a single character, and the actual sequence
        // is escaped by the compiler in a user-supplied string literal.
        let substitutions = vec!["%{C}", "%{D12}", "%{D24}", "%{H}", "%{U}", "\\n"];
        for to_subst in substitutions {
            let mut subst = String::new();
            match to_subst {
                "%{C}" => subst = std::env::current_dir().unwrap().display().to_string(),
                "%{D12}" => subst = get_time("%I:%M %p").to_string(),
                "%{D24}" => subst = get_time("%H:%M").to_string(),
                "%{H}" => subst = self.home.clone(),
                "%{U}" => subst = self.user.clone(),
                "\\n" => subst = '\n'.to_string(), // Needed to support newlines in the prompt.
                _ => ()
            }
            evaled_prompt = evaled_prompt.replace(to_subst, &subst);
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
fn tokenize(input: &str) -> Vec<String> { input.chars().map(|t| t.to_string()).collect::<Vec<String>>() }

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
fn lex_tokenized_input(input: &str) -> Vec<String> {
    let tokenized_vec = tokenize(input);
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
            // Instead of explicitely checking for everything,
            // don't we just append any character that doesn't
            // require extra work, such as quotations.
            _ => {
                if quoted {
                    quoted_vec.push(character.to_string());
                } else {
                    tmp_vec.push(character.to_string());
                    // Needed to push the last element to lexed_vec.
                    if idx == tokenized_vec.len() - 1 {
                        push_to_vec(&mut tmp_vec, &mut lexed_vec);
                    }
                }
            }
        }
    }
    lexed_vec
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
            redirection: get_redirection_type(input)
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
    pub commands: Vec<ShellCommand>
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
                    redirection: get_redirection_type(input)
                };
                commands.push(command);
            } else {
                let command = ShellCommand {
                    name: part[0].clone(),
                    args: part[1..].to_vec(),
                    redirection: get_redirection_type(input)
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

/// Get the calculator vars (`math_op`, `first_number`, `second_number`) for
/// calc.
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
        process_input(shell_state, &input);
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
                "cat" => output_prev = cat(&command.args.clone()),
                "echo" => output_prev = echo(&command.args.clone()),
                "calc" => output_prev = calc(&command.args.clone()),
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
            return String::new();
        },
        Redirection::Append => {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(file_path)
                .unwrap();
            writeln!(file, "{}", output_prev).unwrap();
            return String::new();
        },
        Redirection::NoOp => ()
    }
    match pipe.commands[pipe.commands.len() - 1].name.as_str() {
        "cat" => cat(&pipe.commands[pipe.commands.len() - 1].args.clone()),
        "echo" => echo(&pipe.commands[pipe.commands.len() - 1].args.clone()),
        "calc" => calc(&pipe.commands[pipe.commands.len() - 1].args.clone()),
        "ls" => ls(pipe.commands[pipe.commands.len() - 1].args.clone()),
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
                        Err(why) => return format!("ERROR: could not read cmd2 stdout: {}", why),
                        Ok(_) => output
                    }
                },
                Err(_) => pipe.commands[pipe.commands.len() - 1].name.clone()
            }
        }
    }
}
