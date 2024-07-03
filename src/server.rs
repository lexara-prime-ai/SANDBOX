use reqwest::{self, Client};

#[derive(Debug)]
pub struct Server;

impl Server {
    pub fn new() -> Client {
        Client::new()
    }

    pub fn create_request() {
        // Server::new().post(url)
        todo!()
    }
}
