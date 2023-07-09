use crate::lib::s3::utils::response_error::create_error;
use crate::lib::s3::{client::client::create_client, utils::response_error::ResponseError};
use crate::lib::tauri::operations::show_folder::show_folder;
use aws_sdk_s3::Client;
use serde::{Deserialize, Serialize};
use std::fs::write;

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

    for key in &keys {
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
        let cloned_keys = keys.clone();
        let first_key = cloned_keys.first().unwrap();

        match show_folder(format!(
            "{}/{}",
            dir.to_string(),
            first_key.key.split("/").last().unwrap().to_string()
        ))
        .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(create_error(
                    "Failed to open folder, but files were saved".into(),
                    err.to_string(),
                ))
            }
        }
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
) -> Result<(), ResponseError> {
    let resp = match client
        .get_object()
        .bucket(bucket_name)
        .key(key.to_string())
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return Err(create_error("Failed to get object".into(), e.to_string())),
    };
    let data = match resp.body.collect().await {
        Ok(data) => data,
        Err(e) => {
            return Err(create_error(
                "Failed to collect response body".into(),
                e.to_string(),
            ))
        }
    };
    let path = format!("{}/{}", dir, key.split("/").last().unwrap().to_string());
    match write(path, data.into_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => return Err(create_error("Failed to write path".into(), e.to_string())),
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    // This test will fail because the bucket name and file name are invalid
    #[tokio::test]
    async fn test_save_files_error() {
        let result = save_files(
            vec![FilesToDownload {
                key: "test".into(),
                bucket_name: "test".into(),
            }],
            "test".into(),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_save_files() {
        let result = save_files(
            vec![FilesToDownload {
                key: "website/dockercli.jpeg".into(),
                bucket_name: "spaceout-backup".into(),
            }],
            "./".into(),
        )
        .await;
        assert!(result.is_ok());
    }
}
