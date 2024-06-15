use std::path::PathBuf;

use crate::{
    downloader::{self, VideoDownloadRequest},
    utils::{get_port, get_save_dir, get_save_path},
};
use salvo::prelude::*;
use salvo::serve_static::StaticDir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DeleteRequest {
    pub file_id: String,
}

#[handler]
async fn download_all(req: &mut salvo::Request, res: &mut salvo::Response) {
    let download_request_body = req.parse_json::<Vec<VideoDownloadRequest>>().await;

    if download_request_body.is_err() {
        res.status_code(StatusCode::BAD_REQUEST);
        return;
    }

    let download_requests = download_request_body.unwrap();
    let mut queue = downloader::DownloaderQueue::new(false);

    for download_request in download_requests {
        queue.add(download_request);
    }

    let raw_results = downloader::work_queue(&mut queue).await;

    if raw_results.is_err() {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        return;
    }

    res.render(Json(raw_results.unwrap()));
}

#[handler]
async fn delete(req: &mut salvo::Request, res: &mut salvo::Response) {
    let delete_request_body = req.parse_json::<DeleteRequest>().await;

    println!("{:?}", delete_request_body);

    if delete_request_body.is_err() {
        res.status_code(StatusCode::BAD_REQUEST);
        return;
    }

    let delete_request = delete_request_body.unwrap();

    let save_path = get_save_path(&delete_request.file_id);

    if PathBuf::from(&save_path).exists() {
        std::fs::remove_file(save_path).unwrap();
    }

    res.status_code(StatusCode::NO_CONTENT);
}
pub async fn start_server(file_server: bool) -> eyre::Result<()> {
    let router = match file_server {
        true => Router::with_path("<**path>").get(StaticDir::new(vec![get_save_dir()])),
        false => Router::new().get(download_all).post(delete),
    };
    let addr = format!("0.0.0.0:{}", get_port());

    println!("Listening on {}", addr);

    let acceptor = TcpListener::new(addr).bind().await;

    let server = salvo::Server::new(acceptor);

    server.serve(router).await;

    Ok(())
}
