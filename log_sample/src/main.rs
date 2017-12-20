#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();

    info!("Starting up");
    error!("Error");
    println!("Hello, world!");
}
