mod app;
mod requester;
mod banner;

use app::App;

#[tokio::main]
async fn main(){
    match App::new().run().await{
        Ok(_) => {},
        Err(e) => eprintln!("{e}")
    }
}
