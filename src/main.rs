use std::env;

pub mod utils;
pub mod downloader;
pub mod web;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    utils::setup();


    let is_file_server = env::var("FILE_SERVER").is_ok();

    web::start_server(is_file_server).await?;


    Ok(())


}
