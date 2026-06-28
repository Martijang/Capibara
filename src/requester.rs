use anyhow::{Ok, Result};
use reqwest::Client;

use capibara::request::{Method, RequestResult};

#[derive(Debug)]
pub struct Requester {
    client: Client,
}

impl Requester {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn request(&self, url: &str, method: &Option<Method>) -> Result<RequestResult> {
        if let Some(method) = method {
            match method {
                Method::Get => {
                    let res = self.get(url).await?;
                    Ok(RequestResult {
                        status: res.status,
                        body: res.body,
                    })
                }
                Method::Post => {
                    let res = self.post(url).await?;
                    Ok(RequestResult {
                        status: res.status,
                        body: res.body,
                    })
                }
            }
        } else {
            let res = self.get(url).await?;
            Ok(RequestResult {
                status: res.status,
                body: res.body,
            })
        }
    }

    async fn get(&self, url: &str) -> Result<RequestResult> {
        let res = self.client.get(url).send().await?;
        Ok(RequestResult {
            status: res.status(),
            body: res.text().await?,
        })
    }

    async fn post(&self, url: &str) -> Result<RequestResult> {
        let res = self.client.post(url).send().await?;
        Ok(RequestResult {
            status: res.status(),
            body: res.text().await?,
        })
    }
}
