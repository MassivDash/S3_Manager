use crate::lib::s3::utils::response_error::create_error;
use crate::lib::s3::{client::client::create_client, utils::response_error::ResponseError};
use aws_sdk_s3::Client;
use serde::{Deserialize, Serialize};
use std::{fs::write, io::Error};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilesToDownload {
    pub key: String,
    pub bucket_name: String,
}

#[tauri::command]
pub async fn save_files(keys: Vec<FilesToDownload>, dir: String) -> Result<bool, ResponseError> {
    let client_result = create_client().await;
    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            return Err(create_error(
                "Aws client config error".into(),
                err.to_string(),
            ))
        }
    };

    let mut file_errors: Vec<String> = Vec::new();

    for key in keys {
        let result = save_file(
            &key.key,
            &client,
            dir.to_string(),
            key.bucket_name.to_string(),
        )
        .await;
        let _ops = match result {
            Ok(()) => (),
            Err(err) => file_errors.push(format!("{}:{}", &key.key, err.to_string())),
        };
    }

    if file_errors.is_empty() {
        Ok(true)
    } else {
        let error_string = file_errors.join(", ");
        Err(create_error("file".into(), error_string))
    }
}

pub async fn save_file(
    key: &String,
    client: &Client,
    dir: String,
    bucket_name: String,
) -> Result<(), Error> {
    let resp = client
        .get_object()
        .bucket(bucket_name)
        .key(key.to_string())
        .send()
        .await
        .unwrap();
    let data = resp.body.collect().await.unwrap();
    let path = format!("{}/{}", dir, key.split("/").last().unwrap().to_string());
    write(path, data.into_bytes()).unwrap();
    Ok(())
}
