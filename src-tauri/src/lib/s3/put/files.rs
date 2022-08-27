use crate::lib::s3::client::client::create_client;
use aws_sdk_s3::{types::ByteStream, Client};
use std::{error::Error, fs, path::Path};
use tauri::Window;
use walkdir::WalkDir;

pub async fn put_file(
    client: &Client,
    bucket_name: String,
    file_name: String,
    key: String,
) -> Result<(), Box<dyn Error>> {
    let body = ByteStream::from_path(Path::new(&file_name)).await;
    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body.unwrap())
        .send()
        .await?;

    Ok(())
}

#[tauri::command]
pub async fn put_files(
    window: Window,
    bucket_name: String,
    folder_name: String,
    files: Vec<String>,
) -> bool {
    let client = create_client().await.unwrap();
    for file in files {
        let path = fs::metadata(&file).unwrap();
        if path.is_dir() {
            for fil in WalkDir::new(&file).into_iter().filter_map(|fil| fil.ok()) {
                if fil.metadata().unwrap().is_file() {
                    let filename = &fil
                        .path()
                        .display()
                        .to_string()
                        .split("/")
                        .last()
                        .unwrap_or_default()
                        .to_string();

                    let key = folder_name.clone() + "/" + filename;
                    println!("{} {}", filename, !filename.starts_with("."));

                    if !filename.starts_with(".") {
                        put_file(
                            &client,
                            bucket_name.to_string(),
                            fil.path().display().to_string(),
                            key,
                        )
                        .await
                        .unwrap();

                        window.emit("event-upload-file", &filename).unwrap()
                    }
                }
            }
        } else {
            let filename = &file.split("/").last().unwrap_or_default().to_string();
            let key = folder_name.to_string() + "/" + filename;
            put_file(&client, bucket_name.to_string(), file, key.to_string())
                .await
                .unwrap();

            window.emit("event-upload-file", &filename).unwrap()
        }
    }
    window.emit("event-resync", "upload successful").unwrap();
    return true;
}
