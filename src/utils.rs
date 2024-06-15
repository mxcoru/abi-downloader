use crate::downloader::FileId;
use std::{env, fs, path::Path};

pub static BASE_DIR: &str = "DOWNLOADER_BASE_DIR";

pub fn generate_file_id() -> FileId {
    uuid::Uuid::new_v4().to_string()
}

pub fn get_video_id(url: &str) -> Option<String> {
    let url = url.to_string();

    if !url.contains("youtube.com") && !url.contains("youtu.be") {
        return None;
    }

    Some(cleanup_url(&url))
}

pub fn get_save_path(file_id: &FileId) -> String {
    let save_path = get_save_dir();

    format!("{}/{}.mp3", &save_path, &file_id)
}

pub fn get_download_path(file_id: &FileId) -> String {
    let save_path = get_download_dir();

    format!("{}/{}.webm", &save_path, &file_id)
}

pub fn setup() {
    let save_dir = get_save_dir();
    let download_dir = get_download_dir();

    if !(Path::new(&save_dir)).exists() {
        fs::create_dir_all(save_dir).unwrap();
    }

    if !(Path::new(&download_dir)).exists() {
        fs::create_dir_all(download_dir).unwrap();
    }
}

pub fn get_save_dir() -> String {
    let base_dir = get_base_dir();

    format!("{}/save", &base_dir)
}

pub fn get_port() -> String {
    env::var("PORT").unwrap_or("8080".to_string())
}

fn get_base_dir() -> String {
    env::var(BASE_DIR).unwrap_or("/home/mxcoru/projects/abi-downloader/downloader".to_string())
}

fn get_download_dir() -> String {
    let get_save_dir = get_base_dir();

    format!("{}/download", &get_save_dir)
}

fn cleanup_url(url: &str) -> String {
    let mut url = url
        .replace("https://", "")
        .replace("www.youtube.com/watch?v=", "")
        .replace("youtu.be/", "");

    if url.contains('&') {
        let new_url = url.split('&').collect::<Vec<&str>>();
        url = new_url[0].to_string();
    }

    if url.contains('?') {
        let new_url = url.split('?').collect::<Vec<&str>>();
        url = new_url[0].to_string();
    }

    url.to_string()
}
