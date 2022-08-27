use crate::lib::s3::client::client::create_client;
use aws_sdk_s3::Client;
use tauri::Window;

#[tauri::command]
pub async fn put_folder(window: Window, bucket_name: String, key: String) -> bool {
    let client: Client = create_client().await.unwrap();
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
    return true;
}
