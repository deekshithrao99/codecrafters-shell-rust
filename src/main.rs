#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

fn main() {
    // Uncomment this block to pass the first stage
    

    // Wait for user input
    let stdin = io::stdin();
    

    loop {
        let mut input = String::new();
        print!("$ ");  
    io::stdout().flush().unwrap();
    stdin.read_line(&mut input).unwrap();
    input.pop();
    //check if input is empty or unreconisged
    if input.is_empty() {
        continue;
    } else if input == "exit 0" {
        break;
        
    }
    
    println!("{}: command not found",input.trim());
        
    }
   
}