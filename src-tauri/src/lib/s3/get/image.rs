use crate::lib::s3::utils::presigned_url::get_presigned_url;
use crate::lib::s3::utils::response_error::create_error;
use crate::lib::s3::{client::client::create_client, utils::response_error::ResponseError};

use aws_sdk_s3::model::ObjectAttributes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SingleImgObject {
    pub key: String,
    pub url: String,
    pub size: i64,
    pub last_modified: i64,
}

#[tauri::command]
pub async fn get_image(bucket: String, key: String) -> Result<SingleImgObject, ResponseError> {
    let client_call = create_client().await;

    let client = match client_call {
        Ok(instance) => instance,
        Err(_err) => {
            return Err(create_error(
                "client".into(),
                "Aws client connection failed".into(),
            ))
        }
    };

    let get_image_attributes_result = client
        .get_object_attributes()
        .bucket(&bucket)
        .object_attributes(ObjectAttributes::ObjectSize)
        .key(&key)
        .send()
        .await;

    let img_atr = match get_image_attributes_result {
        Ok(get_image_atr) => get_image_atr,
        Err(err) => return Err(create_error("call".into(), err.to_string())),
    };

    let img = SingleImgObject {
        key: key.to_string(),
        url: get_presigned_url(&client, &bucket, &key, 900)
            .await
            .unwrap(),
        size: img_atr.object_size(),
        last_modified: img_atr.last_modified().unwrap().secs(),
    };

    Ok(img)
}
