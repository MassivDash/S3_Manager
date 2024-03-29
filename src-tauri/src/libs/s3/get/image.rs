use crate::libs::s3::utils::presigned_url::get_presigned_url;
use crate::libs::s3::utils::response_error::create_error;
use crate::libs::s3::{client::client::create_client, utils::response_error::ResponseError};

use aws_sdk_s3::types::ObjectAttributes;
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
        Err(err) => {
            return Err(create_error(
                "AWS Client Config error".into(),
                err.to_string(),
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
        Err(err) => {
            return Err(create_error(
                "Get object attributes call failed".into(),
                err.to_string(),
            ))
        }
    };

    let img = SingleImgObject {
        key: key.to_string(),
        url: get_presigned_url(&client, &bucket, &key, 900)
            .await
            .unwrap(),
        size: img_atr.object_size().expect("no size"),
        last_modified: img_atr.last_modified().unwrap().secs(),
    };

    Ok(img)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_image() {
        let result = get_image(
            "spaceout-backup".to_string(),
            "website/dockercli.jpeg".to_string(),
        )
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_image_error() {
        let result = get_image(
            "spaceout-error".to_string(),
            "website/dockercli.jpeg".to_string(),
        )
        .await;
        assert!(result.is_err());
    }
}
