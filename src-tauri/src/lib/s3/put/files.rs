use aws_sdk_s3::{Client, types::ByteStream};
use crate::lib::s3::client::client::create_client;
use std::{path::Path, error::Error};

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

    println!("Uploaded file: {}", file_name);
    Ok(())
}

#[tauri::command]
pub async fn put_files(
    bucket_name: String,
    folder_name: String,
    files: Vec<String>,
) -> bool {
    let client = create_client().await.unwrap();
    println!("Uploading files to bucket: {}", bucket_name);
    for file in files {
        let key = folder_name.to_string() + "/" + &file.split("/").last().unwrap_or_default().to_string();
        put_file(&client, bucket_name.to_string(), file, key).await.unwrap();
    }
    return true;
}
