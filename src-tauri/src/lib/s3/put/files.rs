use crate::lib::s3::utils::get_file_name::get_file_name;
use crate::lib::s3::utils::response_error::create_error;
use crate::lib::s3::{client::client::create_client, utils::response_error::ResponseError};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
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
) -> Result<bool, ResponseError> {
    let client_call = create_client().await;

    let client = match client_call {
        Ok(instance) => instance,
        Err(err) => {
            return Err(create_error(
                "AWS Client Config error".into(),
                err.to_string(),
            ))
        }
    };

    for file in files {
        let path = fs::metadata(&file).unwrap();
        if path.is_dir() {
            for fil in WalkDir::new(&file).into_iter().filter_map(|fil| fil.ok()) {
                if fil.metadata().unwrap().is_file() {
                    let filename = get_file_name(&fil.path().to_str().unwrap_or_default());
                    let key = folder_name.clone() + "/" + filename;
                    println!("{} {}", filename, !filename.starts_with("."));

                    if !filename.starts_with(".") {
                        match put_file(
                            &client,
                            bucket_name.to_string(),
                            fil.path().display().to_string(),
                            key,
                        )
                        .await
                        {
                            Ok(_) => {
                                window.emit("event-upload-file", &filename).unwrap();
                            }
                            Err(err) => {
                                return Err(create_error(
                                    "Error uploading file".into(),
                                    err.to_string(),
                                ));
                            }
                        }
                    }
                }
            }
        } else {
            let filename = get_file_name(&file);

            let key = folder_name.to_string() + "/" + filename;
            match put_file(
                &client,
                bucket_name.to_string(),
                file.clone(),
                key.to_string(),
            )
            .await
            {
                Ok(_) => {
                    window.emit("event-upload-file", &filename).unwrap();
                }
                Err(err) => {
                    return Err(create_error("Error uploading file".into(), err.to_string()));
                }
            }
        }
    }
    window.emit("event-resync", "upload successful").unwrap();
    return Ok(true);
}
