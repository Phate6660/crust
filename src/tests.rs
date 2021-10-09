#[cfg(test)]
mod tests {
    use crate::builtins::{calc, echo, ls};

    fn calc_run(problem: Vec<String>, solution: String) {
        let output = calc(problem);
        let output = output.trim();
        assert_eq!(output, solution);
    }

    #[test]
    fn basic_calc_add() {
        let problem: Vec<String> = vec!("1+1".to_string());
        calc_run(problem, "2".to_string());
    }

    #[test]
    fn basic_calc_sub() {
        let problem: Vec<String> = vec!("2-1".to_string());
        calc_run(problem, "1".to_string());
    }

    #[test]
    fn basic_calc_mul() {
        let problem: Vec<String> = vec!("2x4".to_string());
        calc_run(problem, "8".to_string());
    }

    #[test]
    fn basic_calc_div() {
        let problem: Vec<String> = vec!("6/3".to_string());
        calc_run(problem, "2".to_string());
    }

    #[test]
    fn basic_echo() {
        let arg: Vec<String> = vec![String::from("Success")];
        let output = echo(arg);
        // TODO: Trim output from any function that outputs by default,
        // unless the extra whitespace or newlines are needed. Currently
        // trimming per test-basis right now to make tests succeed.
        let output = output.trim();
        assert_eq!(output, String::from("Success"));
    }

    #[test]
    fn basic_ls_test() {
        let directory = vec![String::from("src/")];
        let output = ls(directory);
        let output = output.trim();
        let expected = "\u{1b}[32msrc\u{1b}[0m/\u{1b}[37mtests.rs\u{1b}[0m\n\
                        \u{1b}[32msrc\u{1b}[0m/\u{1b}[37mshared_functions.rs\u{1b}[0m\n\
                        \u{1b}[32msrc\u{1b}[0m/\u{1b}[37mbuiltins.rs\u{1b}[0m\n\
                        \u{1b}[32msrc\u{1b}[0m/\u{1b}[37mmain.rs\u{1b}[0m";
        assert_eq!(expected, output);
    }

    #[test]
    fn echo_with_args() {
        let first = String::from("Still");
        let second = String::from("a");
        let third = String::from("success!");
        let args: Vec<String> = vec![first, second, third];
        let output = echo(args);
        let output = output.trim();
        assert_eq!(output, String::from("Still a success!"));
    }
}
