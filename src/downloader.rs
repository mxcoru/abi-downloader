use crate::utils::{self, get_download_path, get_save_path};
use essi_ffmpeg::FFmpeg;
use eyre::OptionExt;
use queued_rust::Queue;
use rusty_ytdl::Video;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

pub type DownloaderQueue = Queue<VideoDownloadRequest>;
pub type FileId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDownloadRequest {
    pub url: String,
    pub start: u16,
    pub end: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDownloadResponse {
    pub file_id: FileId,
    pub url: String,
}

pub async fn work_queue(queue: &mut DownloaderQueue) -> eyre::Result<Vec<VideoDownloadResponse>> {
    let mut responses = Vec::new();

    while let Some(entry) = queue.pop() {
        let file_id = download(&entry).await?;

        let response = VideoDownloadResponse {
            file_id: file_id.clone(),
            url: entry.url.clone(),
        };

        cleanup(&file_id).unwrap();

        responses.push(response);

        sleep(std::time::Duration::from_millis(
            (rand::random::<u8>() as u64
                + rand::random::<u8>() as u64
                + rand::random::<u8>() as u64) as u64,
        ))
        .await;
    }

    Ok(responses)
}

async fn download(entry: &VideoDownloadRequest) -> eyre::Result<FileId> {
    let file_id = download_video(entry).await?;
    cut_mp3(entry, &file_id)?;

    Ok(file_id)
}

fn cleanup(file_id: &FileId) -> eyre::Result<()> {
    let download_path = get_download_path(file_id);

    std::fs::remove_file(download_path)?;
    Ok(())
}

fn cut_mp3(entry: &VideoDownloadRequest, file_id: &FileId) -> eyre::Result<()> {
    let download_path = get_download_path(file_id);
    let save_path = get_save_path(file_id);

    let start_time = entry.start;

    let mut ffmpeg = FFmpeg::new()
        .stderr(std::process::Stdio::inherit())
        .arg("-ss")
        .arg(start_time.to_string())
        .input_with_file((&download_path).into())
        .arg("-t")
        .arg((entry.end - entry.start).to_string())
        .done()
        .output_as_file(save_path.into())
        .done()
        .start()
        .unwrap();

    ffmpeg.wait()?;

    Ok(())
}

async fn download_video(entry: &VideoDownloadRequest) -> eyre::Result<FileId> {
    let video_id = utils::get_video_id(&entry.url).ok_or_eyre("Invalid url")?;

    println!("Downloading video: {}", video_id);

    let video = Video::new(video_id)?;

    let file_id = utils::generate_file_id();
    let file_path = get_download_path(&file_id);

    video.download(&file_path).await?;

    Ok(file_id)
}
