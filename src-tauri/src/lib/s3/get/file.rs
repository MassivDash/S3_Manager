use aws_sdk_s3::Client;
use std::{io::Error, fs::write};

use crate::{lib::s3::client::client::create_client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilesToDownload {
    pub key: String,
    pub bucket_name: String
}

#[tauri::command]
pub async fn save_files(keys: Vec<FilesToDownload>, dir: String) -> bool {
    let client = create_client().await.unwrap();
    for key in keys {
        save_file(key.key, &client, dir.to_string(), key.bucket_name.to_string()).await.unwrap();
    }
    return true;
}

pub async fn save_file(key: String, client: &Client, dir: String, bucket_name: String) -> Result<(), Error> {
    let resp = client
        .get_object()
        .bucket(bucket_name)
        .key(key.to_string())
        .send()
        .await.unwrap();
    let data = resp.body.collect().await.unwrap();
    let path = format!("{}/{}", dir, key.split("/").last().unwrap().to_string());
    write(path, data.into_bytes()).unwrap();
    Ok(())
}