use crate::shared_functions::{
    get_calc_vars, piped_cmd, PipedShellCommand, Redirection, ShellCommand
};

/// Takes the `args` part of a ShellCommand struct and tries
/// to evaluate the given mathematical expression, 
/// returning a String with the result.
pub fn calc(args: Vec<String>) -> String {
    let mut output = String::new();
    if args.contains(&"|".to_string()) {
        let command = ShellCommand {
            name: "calc".to_string(),
            args,
            redirection: Redirection::NoOp,
        };
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else if args.contains(&">>".to_string()) {
        let command = ShellCommand {
            name: "calc".to_string(),
            args,
            redirection: Redirection::Append,
        };
        let pipe = PipedShellCommand::from(command);
        piped_cmd(pipe);
    } else if args.contains(&">".to_string()) {
        let command = ShellCommand {
            name: "calc".to_string(),
            args,
            redirection: Redirection::Overwrite,
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
