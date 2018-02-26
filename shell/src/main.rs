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

pub trait Strategy {
    fn execute(&self, args: &mut Vec<String>);
}

////////////////////////////////////////////////////////
struct RmStrategy {}

impl RmStrategy {
    pub fn new() -> RmStrategy {
        RmStrategy{}
    }

    fn rm_command(filename: &str, args: &mut Vec<String>) {
        let action = |file: &str| fs::remove_file(file);
        if args.len() > 0 {
            match args[0].as_str() {
                "-r" => {
                    match fs::remove_dir_all(filename) {
                        Ok(_) => {},
                        Err(error) => println!("rm: cannot remove '{}': Is a directory", filename)
                    }
                }
                _ => {}
            }
        } else {
            match fs::remove_file(filename) {
                Ok(_) => {},
                Err(error) => println!("rm: cannot remove '{}': Is a directory", filename)
            };
        }
    }
}

impl Strategy for RmStrategy {
    fn execute(&self, mut args: &mut Vec<String>) {
        RmStrategy::rm_command(args.remove(0).as_str(), args);
    }
}
/////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////
struct CatStrategy {}

use std::io::Read;
use std::fs::File;
impl CatStrategy {
    pub fn new() -> CatStrategy {
        CatStrategy{}
    }

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
}

impl Strategy for CatStrategy {
    fn execute(&self, mut args: &mut Vec<String>) {
        CatStrategy::cat_command(args.remove(0).as_str());
    }
}
/////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////
struct MkdirStrategy {}

impl MkdirStrategy {
    pub fn new() -> MkdirStrategy {
        MkdirStrategy{}
    }

    fn mkdir_command(path: &str) {
        match fs::create_dir(path) {
            Ok(_) => {},
            Err(_) => println!("mkdir: cannot create directory '{}': File exists", path)
        };
    }
}

impl Strategy for MkdirStrategy {
    fn execute(&self, mut args: &mut Vec<String>) {
        MkdirStrategy::mkdir_command(args.remove(0).as_str());
    }
}
/////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////
struct CdStrategy {}

use std::env;
impl CdStrategy {
    pub fn new() -> CdStrategy {
        CdStrategy{}
    }

    fn cd_command(path: &str) {
        let new_path = Path::new(path);
        env::set_current_dir(&new_path).is_ok();
    }
}

impl Strategy for CdStrategy {
    fn execute(&self, mut args: &mut Vec<String>) {
        CdStrategy::cd_command(args.remove(0).as_str());
    }
}
/////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////
struct HelpStrategy {}

use std::path::Path;
impl HelpStrategy {
    pub fn new() -> HelpStrategy {
        HelpStrategy{}
    }

    fn help_command() {
        println!("Rust bash, version 1.00");
        println!("These shell commands are defined interally. Type `help` to see this list.");
        println!("A star (*) next to a name means that the command is disabled\n");

        println!("help\t\t\tDisplays the help menu");
        println!("echo\t\t\tOutputs the passed in message");
    }
}

impl Strategy for HelpStrategy {
    fn execute(&self, mut args: &mut Vec<String>) {
        HelpStrategy::help_command();
    }
}
/////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////
struct LsStrategy {}
use std::fs;
impl LsStrategy {
    pub fn new() -> LsStrategy {
        LsStrategy{}
    }

    fn ls_command() {
        let paths = fs::read_dir("./").unwrap();

        for path in paths {
            print!("{}\t", path.unwrap().path().display());
        }
        println!("");
    }
}

impl Strategy for LsStrategy {
    fn execute(&self, mut args: &mut Vec<String>) {
        LsStrategy::ls_command();
    }
}
/////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////
struct EchoStrategy {}

impl EchoStrategy {
    pub fn new() -> EchoStrategy {
        EchoStrategy{}
    }

    fn echo_command(message: &str) {
        println!("{}", message);
    }
}

impl Strategy for EchoStrategy {
    fn execute(&self, mut args: &mut Vec<String>) {
        EchoStrategy::echo_command(args.remove(0).as_str());
    }
}
/////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////
struct TouchStrategy {}

impl TouchStrategy {
    pub fn new() -> TouchStrategy {
        TouchStrategy{}
    }

    fn touch_command(filename: &str) {
        match File::open(filename) {
            Ok(f) => {},
            Err(open_error) => { match File::create(filename) {
                Ok(f) => {},
                Err(create_error) => {}
            }}
        };
    }
}

impl Strategy for TouchStrategy {
    fn execute(&self, mut args: &mut Vec<String>) {
        TouchStrategy::touch_command(args.remove(0).as_str());
    }
}
/////////////////////////////////////////////////////////////////////////////////////



/////////////////////////////////////////////////////////////////////////////////////
struct UnknownStrategy {}

impl UnknownStrategy {
    pub fn new() -> UnknownStrategy {
        UnknownStrategy{}
    }
}

impl Strategy for UnknownStrategy {
    fn execute(&self, mut args: &mut Vec<String>) {
    }
}
/////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////
struct ExitStrategy {}

impl ExitStrategy {
    pub fn new() -> ExitStrategy {
        ExitStrategy{}
    }

    fn exit_command() {
        println!("Not yet implemented!");
    }
}

impl Strategy for ExitStrategy {
    fn execute(&self, mut args: &mut Vec<String>) {
        ExitStrategy::exit_command();
    }
}
/////////////////////////////////////////////////////////////////////////////////////

struct Command {
    args: Vec<String>,
    strategy: Box<Strategy>
}

impl Command {
    pub fn new() -> Command {
        Command {
            strategy: Box::new(UnknownStrategy::new()),
            args: Vec::new()
        }
    }

    pub fn parse(&mut self, command: &str, args: Vec<&str>) {
        self.strategy = match command {
            "help" => Box::new(HelpStrategy::new()),
            "echo" => Box::new(EchoStrategy::new()),
            "ls" => Box::new(LsStrategy::new()),
            "touch" => Box::new(TouchStrategy::new()),
            "cd" => Box::new(CdStrategy::new()),
            "mkdir" => Box::new(MkdirStrategy::new()),
            "cat" => Box::new(CatStrategy::new()),
            "exit" => Box::new(ExitStrategy::new()),
            "rm" => Box::new(RmStrategy::new()),
            _ => Box::new(UnknownStrategy::new())
        };
        args.iter().map(|arg| self.args.push(String::from(*arg)));
    }

    pub fn execute(&mut self) {
        self.strategy.execute(&mut self.args)
    }
}


fn parse_command(command: &str, mut args: Vec<&str>) {
    let mut new_command = Command::new();
    new_command.parse(command, args);
    new_command.execute();
}