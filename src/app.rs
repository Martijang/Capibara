use clap::{Parser, ValueEnum};

use crate::{banner::BannerMaker, requester::Requester};

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
enum Method{
    Get,
    Post,
}

#[allow(unused)]
///Basic GET/POST requester written in rust
#[derive(Parser, Debug)]
#[clap(about, long_about = None, version)]
struct Cli{
    ///Url(s) to send request
    #[clap(num_args =1..)]
    urls: Vec<String>,

    ///Method to use. Default is GET
    #[arg(short, long, value_enum)]
    method: Option<Method>,

    ///print status code only. Default is No 
    #[arg(short, long)]
    status: Option<bool>
}


pub struct App{
    args: Cli,
    req: Requester,
    banner: BannerMaker
}

impl App{
    pub fn new() -> Self{
        Self { args: Cli::parse(), req: Requester::new(), banner: BannerMaker::new() }
    }

    pub async fn run(&mut self){
        self.banner.print_banner();
    }
}
