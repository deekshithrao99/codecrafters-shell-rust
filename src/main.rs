#[allow(unused_imports)]
use std::io::{self, Write};
use std::{process::Command, string, vec};

fn main() {

    // Wait for user input
    let stdin = io::stdin();

    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        //reading the input
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        //check if input is empty or unreconisged
        if input.is_empty() {
            continue;
        } else if input == "exit 0" {
            break;
        }
        let mut command_split = input.split_whitespace();
        let command = command_split.next().unwrap_or("");


        match command {
            "echo" => {
                let echo_arg: Vec<String> = command_split.map(String::from).collect();
                println!("{}",echo_arg.join(" "));
            }
            

            "type" => {
               
                match command_split.next() {
                    Some(subcommand) => match subcommand {
                    "echo" => println!("echo is a shell builtin"),
                    "exit" => println!("exit is a shell builtin"),
                    "type" => println!("type is a shell builtin"),
                    
                    _ => println!("{}: not found",subcommand)
                    },
                    None => {
                        println!("type: missing argument");
                    }

                    }
                    
                   
                
            }

            _=> {
                println!("{}: command not found",input.trim());
            }
        }
    }
}
