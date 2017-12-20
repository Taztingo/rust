fn main() {
    let ip:String =  String::from("127.0.0.1");
    let port:u16 = 8799;
    let bytes:u16 = 1024;
    let mut server = Server::new(ip, port);
    server.start();
}



#[macro_use]
extern crate log;
use std::net::UdpSocket;
struct Server {
    running: bool,
    ip: String,
    port: u16,
}

impl Server {
    pub fn new(ip: String, port: u16) -> Server {
        Server {
            running: false,
            ip: ip,
            port: port
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
        let log = MessageParser::new(message);
        if log.message_type == "server" {
            self.parse_server_message(log.message);
        }
    }

    fn parse_server_message(&mut self, message: String) {
        if message == "exit" {
            self.running = false;
        } else if message == "trace_level" {

        } else if message == "debug_level" {

        } else if message == "info_level" {

        } else if message == "warn_level" {

        } else if message == "error_level" {

        }
    }
}

struct MessageParser {
    message_type: String,
    message: String,
    level: String
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
            Some(arg) => arg,
            None => ""
        };
        MessageParser {
            message_type: message_type.to_string(),
            message: message.to_string(),
            level: level.to_string()
        }
    }
}