use aws_sdk_s3::Client;

use crate::libs::s3::utils::response_error::create_error;
use crate::libs::s3::{client::client::create_client, utils::response_error::ResponseError};
use serde::{Deserialize, Serialize};
use tauri::Window;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilesToDelete {
    pub key: String,
    pub bucket_name: String,
}

#[tauri::command]
pub async fn delete_files(window: Window, keys: Vec<FilesToDelete>) -> Result<bool, ResponseError> {
    let client_result = create_client().await;
    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            println!("{}", err.to_string());
            return Err(create_error(
                "Aws client config error".into(),
                err.to_string(),
            ));
        }
    };
    for key in keys {
        match remove_file(&client, &key.bucket_name, &key.key).await {
            Ok(_) => (),
            Err(err) => {
                println!("{}", err.to_string());
                return Err(create_error(
                    format!("Failed to delete file {}", &key.key),
                    err.to_string(),
                ));
            }
        }
    }
    window.emit("event-resync", "delete successful").unwrap();
    return Ok(true);
}

async fn remove_file(client: &Client, bucket: &str, key: &str) -> Result<(), ResponseError> {
    match client.delete_object().bucket(bucket).key(key).send().await {
        Ok(_) => Ok(()),
        Err(err) => Err(create_error(
            "Failed to delete file".into(),
            err.to_string(),
        )),
    }
}
