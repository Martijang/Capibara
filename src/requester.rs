use reqwest::Client;

#[derive(Debug)]
pub struct Requester{
    client: Client
}

impl Requester{
    pub fn new() -> Self{
        Self{ client: Client::new() }
    }
}
