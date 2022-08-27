use aws_sdk_s3::Client;
use std::io::Error;

use crate::lib::s3::client::client::create_client;
use serde::{Deserialize, Serialize};
use tauri::Window;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilesToDelete {
    pub key: String,
    pub bucket_name: String,
}

#[tauri::command]
pub async fn delete_files(window: Window, keys: Vec<FilesToDelete>) -> bool {
    let client = create_client().await.unwrap();
    for key in keys {
        remove_file(&client, &key.bucket_name, &key.key)
            .await
            .unwrap();
    }
    window.emit("event-resync", "delete successful").unwrap();
    return true;
}

async fn remove_file(client: &Client, bucket: &str, key: &str) -> Result<(), Error> {
    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .unwrap();

    Ok(())
}
