use crate::shared_functions::is_piped;
use std::fs::File;
use std::io::{BufReader, Read};

fn read_file(file: File) -> String {
    let mut bufreader = BufReader::new(file);
    let mut contents = String::new();
    bufreader.read_to_string(&mut contents).unwrap();
    contents
}

pub fn cat(args: &[String]) -> String {
    is_piped(args, "cat");
    match args[0].as_str() {
        "-n" => {
            let mut final_output = String::new();
            let output = read_file(File::open(args[1].clone()).unwrap());
            let output_vec = output.split('\n');
            for (idx, line) in output_vec.enumerate() {
                let string = format!("{} {}\n", idx, line);
                final_output.push_str(&string);
            }
            final_output
        },
        _ => read_file(File::open(args[0].clone()).unwrap())
    }
}
