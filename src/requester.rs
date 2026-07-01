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

    pub async fn request(
        &self,
        url: &str,
        method: &Option<Method>,
        body: &Option<String>,
    ) -> Result<RequestResult> {
        if let Some(method) = method {
            match method {
                Method::Get => {
                    let res = self.get(url, body).await?;
                    Ok(RequestResult {
                        status: res.status,
                        body: res.body,
                    })
                }
                Method::Post => {
                    let res = self.post(url, body).await?;
                    Ok(RequestResult {
                        status: res.status,
                        body: res.body,
                    })
                }
            }
        } else {
            let res = self.get(url, body).await?;
            Ok(RequestResult {
                status: res.status,
                body: res.body,
            })
        }
    }

    async fn get(&self, url: &str, body: &Option<String>) -> Result<RequestResult> {
        if let Some(content) = body {
            let res = self.client.get(url).body(content.to_owned()).send().await?;

            Ok(RequestResult {
                status: res.status(),
                body: res.text().await?,
            })
        } else {
            let res = self.client.get(url).send().await?;
            Ok(RequestResult {
                status: res.status(),
                body: res.text().await?,
            })
        }
    }

    async fn post(&self, url: &str, body: &Option<String>) -> Result<RequestResult> {
        if let Some(content) = body {
            let res = self
                .client
                .post(url)
                .body(content.to_owned())
                .send()
                .await?;

            Ok(RequestResult {
                status: res.status(),
                body: res.text().await?,
            })
        } else {
            let res = self.client.get(url).send().await?;

            Ok(RequestResult {
                status: res.status(),
                body: res.text().await?,
            })
        }
    }
}

#[cfg(test)]
mod test{
    use reqwest::StatusCode;
    
    use super::*;


    #[tokio::test]
    async fn try_get_request_to_example_com(){
        let requester = Requester::new();

        let status = requester.request("https://example.com", &Some(Method::Get), &None).await.unwrap();
        assert_eq!(StatusCode::OK, status.status)
    }

    #[tokio::test]
    async fn try_post_with_body_to_example_com(){
        let requester = Requester::new();

        let status = requester.request("https://example.com", &Some(Method::Post), &Some(String::from(""))).await.unwrap();
        assert_eq!(StatusCode::METHOD_NOT_ALLOWED, status.status)
    }
}
