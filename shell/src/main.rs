use std::io;
use std::io::Write;

fn main() {
    display_welcome();
    run_main_loop();
}

fn run_main_loop() {
    let running = true;
    while running {
        match get_user_input() {
            Some(input) => {
                parse_line(&input);
            },
            None => {}
        }
    }
}

fn display_welcome() {
    println!("Welcome to the Rust Shell.\n");
}

fn get_user_input() -> Option<String> {
    let mut input = String::new();
    print!(">> ");
    io::stdout().flush();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            input.pop();
            Some(input)
        },
        Err(error) => {
            println!("Error unable to read line. {}", error);
            None
        }
    }
}

fn parse_line(line: &String) {

    parse_command(line, Vec::new());
    //match line {

    //};
}

fn parse_command(command: &String, args: Vec<String>) {
    match command {
        _ => { 
            println!("{}: command not found", command);
            io::stdout().flush();
        }
    }
}