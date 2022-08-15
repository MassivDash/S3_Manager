
use crate::lib::s3::client::client::create_client;
use crate::lib::s3::utils::presigned_url::get_presigned_url;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SingleImgObject {
    pub key: String,
    pub url: String,
}

#[tauri::command]
pub async fn get_image(bucket: String, key: String) -> SingleImgObject {
    let client = create_client().await.unwrap();
    let img = SingleImgObject {
        key: key.to_string(),
        url: get_presigned_url(
            &client,
            &bucket,
            &key,
            900,
        )
        .await
        .unwrap(),
    };
    return img;
}