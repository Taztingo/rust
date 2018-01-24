fn main() {
    let ip:String =  String::from("127.0.0.1");
    let port:u16 = 8799;
    let bytes:u16 = 1024;
    let mut server = Server::new(ip, port);
    server.start();
}

enum LogLevel {
    Trace = 1,
    Debug = 2,
    Info = 3,
    Warning = 4,
    Error = 5
}

impl LogLevel {
    pub fn new(level: &String) -> Result<LogLevel, ServerError> {
        if level == "trace" {
            Ok(LogLevel::Trace)
        } else if level == "debug" {
            Ok(LogLevel::Debug)
        } else if level == "info" {
            Ok(LogLevel::Info)
        } else if level == "warning" {
            Ok(LogLevel::Warning)
        } else if level == "error" {
            Ok(LogLevel::Error)
        } else {
            Err(ServerError::LogLevelUndefined)
        }
    }
}

enum ServerError {
    LogLevelUndefined
}


#[macro_use]
extern crate log;
use std::net::UdpSocket;
struct Server {
    running: bool,
    ip: String,
    port: u16,
    log_level: LogLevel
}

impl Server {
    pub fn new(ip: String, port: u16) -> Server {
        Server {
            running: false,
            ip: ip,
            port: port,
            log_level: LogLevel::Trace
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        let address = format!("{}:{}", self.ip, self.port);
        info!("Commencing yak shaving");
        println!("Binding to socket: {}", address);
        let socket = UdpSocket::bind(address).expect("Couldn't bind to address");
        let mut buf = [0; 256];

        //Loop for reading
        println!("Starting socket read");
        while self.running {
            let (number_of_bytes, source) = socket.recv_from(&mut buf)
            .expect("Didn't receive data");
            let data = Vec::from(&buf[0..number_of_bytes]);
            let message = String::from_utf8(data).expect("Found invalid UTF-8");
            println!("Received message: {}", message);
            self.parse_message(message);
        }
    }

    fn parse_message(&mut self, message: String) {
        let parsed_message = MessageParser::new(message);
        if parsed_message.message_type == "server" {
            match self.parse_server_message(parsed_message.message) {
                Ok(_) => (),
                Err(error) => println!("{} is an unknown log level!", error)
            }
        } else if parsed_message.message_type == "log" {
            //We need to check if the log level is high enough
            //We also need to create the logs here
        }
    }

    fn parse_server_message(&mut self, message: String) -> Result<(), String> {
        if message == "exit" {
            self.running = false;
        } else {
            match LogLevel::new(&message) {
                Ok(arg) => self.log_level = arg,
                Err(error) => return Err(message)
            }
        }
        Ok(())
    }
}

struct MessageParser {
    message_type: String,
    message: String,
    level: u8
}

impl MessageParser {
    pub fn new(message_to_parse: String) -> MessageParser {
        let mut iter = message_to_parse.split(" ");
        let message_type = match iter.next() {
            Some(arg) => arg,
            None => ""
        };
        let message = match iter.next() {
            Some(arg) => arg,
            None => ""
        };
        let level = match iter.next() {
            Some(arg) => arg.parse().unwrap(),
            None => 1
        };
        MessageParser {
            message_type: message_type.to_string(),
            message: message.to_string(),
            level: level
        }
    }
}