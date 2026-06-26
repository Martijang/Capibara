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

    ///print status code only. Default is No 
    #[arg(short, long)]
    status: Option<bool>
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
}
