use crate::lib::s3::utils::response_error::create_error;
use crate::lib::s3::{client::client::create_client, utils::response_error::ResponseError};
use tauri::Window;

#[tauri::command]
pub async fn put_folder(
    window: Window,
    bucket_name: String,
    key: String,
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

    client
        .put_object()
        .bucket(bucket_name)
        .key(key + "/")
        .send()
        .await
        .unwrap();

    window
        .emit("event-resync", "folder creation successful")
        .unwrap();
    return Ok(true);
}
