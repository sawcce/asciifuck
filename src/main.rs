use std::env;
use std::fs::{read_to_string, write};

fn execute(code: String) -> String {
    let mut document = vec![0u8];
    let mut current_index: usize = 0;

    for char in code.chars().into_iter() {
        match char {
            '+' => document[current_index] += 1,
            '>' => {
                current_index += 1;
                document.push(0)
            }
            _ => {}
        }
    }

    return String::from_utf8(document.to_owned()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("ASCII FUCK");

    match args.len() {
        1 => panic!("Expected at least one cli argument!"),
        2 => {
            let arg = args.get(1).unwrap();
            if arg == "help" {
                println!("CLI usage: ");
                println!("  asciifuck input_file");
                println!("  or");
                println!("  asciifuck input_file output_file");
                return;
            }

            let code = read_to_string(arg).expect(&*format!("Couldn't read file \"{arg}\""));

            let output = execute(code);
            println!("{output}");
        }
        3 => {
            let path = args.get(1).unwrap();

            let code = read_to_string(path).expect(&*format!("Couldn't read file \"{path}\""));

            let output = execute(code);

            let destination = args.get(2).unwrap();
            write(destination, output).expect(&*format!("Couldn't write file \"{destination}\""));
        }
        _ => panic!("Too many cli arguments!"),
    }
}
