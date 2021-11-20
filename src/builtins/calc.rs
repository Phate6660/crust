use crate::commands::is_piped;
use crate::shared_functions::get_calc_vars;

/// Takes the `args` part of a `ShellCommand` struct,
/// and tries to evaluate the given mathematical expression,
/// returning a String with the result.
pub fn calc(args: &[String]) -> String {
    let mut output = String::new();
    is_piped(args, "calc");
    let problem = args.concat();
    let (math_op, first_number, second_number) = get_calc_vars(&problem);
    match math_op {
        // Multiplication
        "x" | "*" => output.push_str(format!("{}", first_number * second_number).as_str()),
        // Division
        "/" => output.push_str(format!("{}", first_number / second_number).as_str()),
        // Division with remainder
        "%" => output.push_str(format!("{}", first_number % second_number).as_str()),
        // Addition
        "+" => output.push_str(format!("{}", first_number + second_number).as_str()),
        // Subtraction
        "-" => output.push_str(format!("{}", first_number - second_number).as_str()),
        _ => output.push_str(format!("Error, '{}' is an unsupported operation.", math_op).as_str())
    }
    output
}
