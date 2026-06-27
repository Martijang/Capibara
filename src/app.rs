use clap::Parser;

use std::sync::Arc;
use crate::{banner::BannerMaker, requester::Requester};
use capibara::request::Method;

#[allow(unused)]
///Basic GET/POST requester written in rust
#[derive(Parser, Debug)]
#[clap(about, long_about = None, version)]
struct Cli{
    ///Url(s) to send request
    #[arg(short, long, num_args =1..)]
    urls: Vec<String>,

    ///Method to use. Default is GET
    #[arg(short, long, value_enum)]
    method: Option<Method>,

    ///print response body only. Default is No 
    #[arg(short, long)]
    body: Option<bool>,

    ///read a file and make request for each url.
    ///all urls must be aligned line by line 
    ///(Not implemented)
    #[arg(short, long)]
    input: Option<String>
}

#[allow(unused)]
#[derive(Debug)]
pub struct App{
    args: Arc<Cli>,
    req: Arc<Requester>,
    banner: BannerMaker
}

impl App{
    pub fn new() -> Self{
        Self { 
            args: Arc::new(Cli::parse()), 
            req: Arc::new(Requester::new()), 
            banner: BannerMaker::new()
        }
    }

    pub async fn run(mut self){
        self.banner.print_banner();
        let body = self.args.body.unwrap_or(false);
        if body{
            self.run_out_as_body().await;
        }else{
            self.run_out_as_status().await;
        }
    }
    async fn run_out_as_status(&self){
        let mut t_vec = Vec::new();
        for url in self.args.urls.clone(){  
            let req = Arc::clone(&self.req);
            let arg = Arc::clone(&self.args);

            t_vec.push(tokio::spawn(async move {
                match req.request(&url, &arg.method).await{
                    Ok(req) => println!("url: {} status: {}", &*url, req.status),
                    Err(e) => eprintln!("url: {} {:?}", &*url, e)
                }
            }));
        }

        for thread in t_vec{
            let res = thread.await;
            match res{
                Ok(_) => {},
                Err(e) => eprintln!("Join error {e}"),
            }
        }
    }

    async fn run_out_as_body(&self) {
        let mut t_vec = Vec::new();
        for url in self.args.urls.clone(){  
            let req = Arc::clone(&self.req);
            let arg = Arc::clone(&self.args);

            t_vec.push(tokio::spawn(async move {
                match req.request(&url, &arg.method).await{
                    Ok(req) => println!("url: {}\n\n body: {}", &*url, req.body),
                    Err(e) => eprintln!("url: {} {:?}", &*url, e)
                }
            }));
        }

        for thread in t_vec{
            let res = thread.await;
            match res{
                Ok(_) => {},
                Err(e) => eprintln!("Join error {e}"),
            }
        }
    }
}
