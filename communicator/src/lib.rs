pub mod client;
pub mod network;

#[cfg(test)]
mod tests {

    use super::client;

    #[test]
    fn it_works() {
        //Naive way that starts from root
        //::client::connect();

        //Correct way to go up one module
        //super::client::connect();

        //Simplified with module use
        client::connect();
    }
}
