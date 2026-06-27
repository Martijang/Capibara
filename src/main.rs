mod app;
mod requester;
mod banner;

use app::App;

fn main() -> anyhow::Result<()>{
    let app = App::new();
    let runtime = app.runtime_init()?;

    runtime.block_on(async {
        match app.run().await{
            Ok(_) => {},
            Err(e) => eprintln!("{e}")
        }
    });
    anyhow::Ok(())
}
