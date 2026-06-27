use clap::Parser;
use tokio::{runtime::{Builder, Runtime}, task::JoinHandle};
use anyhow::Result;

use crate::{banner::BannerMaker, requester::Requester};
use capibara::request::Method;

use std::{fs::File, io::{BufRead, BufReader}, sync::Arc};


///basic GET/POST requester written in rust
#[derive(Parser, Debug)]
#[clap(about, long_about = None, version)]
struct Cli{
    ///url(s) to send the request
    #[arg(short, long, num_args =1..)]
    urls: Vec<String>,

    ///method to use. Default is GET
    #[arg(short, long, value_enum)]
    method: Option<Method>,

    ///print response body only. Default is No 
    #[arg(short, long)]
    body: Option<bool>,

    ///read a file and send request for each url.
    ///all urls must be aligned line by line 
    #[arg(short, long)]
    input: Option<String>,

    ///manually setting number of the worker threads. Default is 1.
    ///NOTE: there are no big performance difference
    #[arg(short, long)]
    threads: Option<usize>
}

#[derive(Debug)]
struct CliHolder{
    pub urls: Vec<String>,
    pub method: Arc<Option<Method>>,
    pub body: Option<bool>,
    pub input: Option<String>,
    pub threads: Option<usize>
}

#[derive(Debug)]
pub struct App{
    args: CliHolder,
    req: Arc<Requester>,
    banner: BannerMaker
}

impl App{
    pub fn new() -> Self{
        Self { 
            args: CliHolder::new(), 
            req: Arc::new(Requester::new()), 
            banner: BannerMaker::new()
        }
    }

    pub async fn run(mut self) -> Result<()>{
        self.banner.print_banner();
        self.read_file_by_line()?;
        let body = self.args.body.unwrap_or(false);
        if body{
            self.run_out_as_body().await;
        }else{
            self.run_out_as_status().await;
        }
        Ok(())
    }

    pub fn runtime_init(&self) -> Result<Runtime>{
        if let Some(threads) = self.args.threads{
            return Ok(Builder::new_multi_thread()
                .worker_threads(threads)
                .enable_all()
                .build()?);
        }
        Ok(
            Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()?
          )
    }
}

//private functions
impl App{
    async fn run_out_as_status(&self){
        let mut t_vec = Vec::new();

        for url in self.args.urls.clone(){  
            let req = Arc::clone(&self.req);
            let method = Arc::clone(&self.args.method);

            t_vec.push(tokio::spawn(async move {
                match req.request(&url, &method).await{
                    Ok(req) => println!("url: {} status: {}", &url, req.status),
                    Err(e) => eprintln!("url: {} {:?}", &url, e)
                }
            }));
        }
        App::join(t_vec).await;
    }

    async fn run_out_as_body(&self) {
        let mut t_vec = Vec::new();

        for url in self.args.urls.clone(){  
            let req = Arc::clone(&self.req);
            let method = Arc::clone(&self.args.method);

            t_vec.push(tokio::spawn(async move {
                match req.request(&url, &method).await{
                    Ok(req) => println!("url: {}\nbody:\n{}", &url, req.body),
                    Err(e) => eprintln!("url: {} {:?}", &url, e)
                }
            }));
        }
        App::join(t_vec).await;
    }

    async fn join(t_vec: Vec<JoinHandle<()>>){
        for thread in t_vec{
            let res = thread.await;
            match res{
                Ok(_) => {},
                Err(e) => eprintln!("Join error {e}"),
            }
        }
    }

    fn read_file_by_line(&mut self) -> Result<()>{
        if let Some(path) = &self.args.input{
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            for line in reader.lines(){
                let line = line?.trim().to_string();
                self.args.urls.push(line);
            }
        }
        Ok(())
    }
}

impl CliHolder{
    pub fn new() -> Self{
        let cli = Cli::parse();
        Self { 
            urls: cli.urls, 
            method: Arc::new(cli.method),
            body: cli.body, 
            input: cli.input,
            threads: cli.threads,
        }
    }
}
