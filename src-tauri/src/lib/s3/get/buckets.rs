use serde::{Deserialize, Serialize};

use crate::lib::s3::{
    client::client::create_client,
    utils::response_error::{create_error, ResponseError},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BucketInfo {
    pub name: String,
    pub creation_date: i64,
}

#[tauri::command]
pub async fn get_buckets() -> Result<Vec<BucketInfo>, ResponseError> {
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

    let resp_call = &client.list_buckets().send().await;
    let resp = match resp_call {
        Ok(list) => list,
        Err(err) => {
            return Err(create_error(
                "S3 bucket call failed".into(),
                err.to_string(),
            ))
        }
    };

    let buckets = resp.buckets().unwrap();
    let mut my_buckets = Vec::new();

    for bucket in buckets {
        my_buckets.push(BucketInfo {
            name: bucket.name().unwrap_or_default().to_string(),
            creation_date: bucket.creation_date().unwrap().secs(),
        });
    }
    return Ok(my_buckets);
}
