use reqwest::Client;
use anyhow::{Result, Ok};

use capibara::request::RequestResult;

#[derive(Debug)]
pub struct Requester{
    client: Client
}

impl Requester{
    pub fn new() -> Self{
        Self{ client: Client::new() }
    }

    pub fn request(&self) -> Result<RequestResult>{
        todo!("")
    }

    fn get(&self) -> Result<RequestResult>{
        todo!("")
    }

    fn post(&self) -> Result<RequestResult>{
        todo!("")
    }
}
