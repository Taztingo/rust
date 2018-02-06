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
                parse_line(input);
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

fn parse_line(line: String) {
    let mut args: Vec<&str> = line.split(" ").collect();
    let command = args.remove(0);
    parse_command(command, args);
}

fn help_command() {
    println!("Rust bash, version 1.00");
    println!("These shell commands are defined interally. Type `help` to see this list.");
    println!("A star (*) next to a name means that the command is disabled\n");

    println!("help\t\t\tDisplays the help menu");
    println!("echo\t\t\tOutputs the passed in message");
}

fn echo_command(message: &str) {
    println!("{}", message);
}

use std::fs;
fn ls_command() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        print!("{}\t", path.unwrap().path().display());
    }
    println!("");
}

use std::fs::File;
fn touch_command(filename: &str) {
    match File::open(filename) {
        Ok(f) => {},
        Err(open_error) => { match File::create(filename) {
            Ok(f) => {},
            Err(create_error) => {}
        }}
    };
}

use std::env;
use std::path::Path;
fn cd_command(path: &str) {
    let new_path = Path::new(path);
    env::set_current_dir(&new_path).is_ok();
}


fn mkdir_command(path: &str) {
    match fs::create_dir(path) {
        Ok(_) => {},
        Err(_) => println!("mkdir: cannot create directory '{}': File exists", path)
    };
}

use std::io::Read;
fn cat_command(filename: &str) {
    match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            println!("{}\n", contents);
        },
        Err(_) => {
            println!("File does not exist!");
        }
    }
}

fn exit_command() {
    println!("Not yet implemented!");
}

fn rm_command(filename: &str) {
    match fs::remove_file(filename) {
        Ok(_) => {},
        Err(error) => println!("rm: cannot remove '{}': Is a directory", filename)
    };
}

fn parse_command(command: &str, args: Vec<&str>) {
    match command {
        "help" => {
            help_command();
        },
        "echo" => {
            echo_command(args[0]);
        },
        "ls" => {
            ls_command();
        },
        "touch" => {
            touch_command(args[0]);
        },
        "cd" => {
            cd_command(args[0]);
        },
        "mkdir" => {
            mkdir_command(args[0]);
        },
        "cat" => {
            cat_command(args[0]);
        },
        "exit" => {
            exit_command();
        },
        "rm" => {
            rm_command(args[0]);
        }
        _ => { 
            println!("{}: command not found", command);
            io::stdout().flush();
        }
    }
}