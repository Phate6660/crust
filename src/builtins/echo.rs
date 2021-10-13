use crate::shared_functions::{
    piped_cmd, return_shellcommand, PipedShellCommand, Redirection
};

/// Just like you know it. Takes the args part of ShellCommand and prints them.
pub fn echo(args: Vec<String>) -> String {
    let mut output = String::new();
    if args.contains(&"|".to_string()) {
        let command = return_shellcommand("echo".to_string(), args, Redirection::NoOp);
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else if args.contains(&">>".to_string()) {
        let command = return_shellcommand("echo".to_string(), args, Redirection::Append);
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else if args.contains(&">".to_string()) {
        let command = return_shellcommand("echo".to_string(), args, Redirection::Overwrite);
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else {
        for arg in args {
            output.push_str(format!("{} ", arg).as_str());
        }
    }
    output
}
