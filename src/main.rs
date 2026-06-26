mod app;
mod requester;
mod banner;

use app::App;

#[tokio::main]
async fn main() {
    App::new().run().await;
}
