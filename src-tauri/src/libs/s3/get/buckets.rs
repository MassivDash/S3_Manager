use crate::libs::s3::{
    client::client::create_client,
    utils::response_error::{create_error, ResponseError},
};
use serde::{Deserialize, Serialize};

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
            println!("{}", err.to_string());
            return Err(create_error(
                "AWS Client Config error".into(),
                err.to_string(),
            ));
        }
    };

    let resp_call = &client.list_buckets().send().await;
    let resp = match resp_call {
        Ok(list) => list,
        Err(err) => {
            println!("{}", err.to_string());
            return Err(create_error(
                "S3 bucket call failed".into(),
                err.to_string(),
            ));
        }
    };

    let buckets = resp.buckets();
    let mut my_buckets = Vec::new();

    for bucket in buckets {
        my_buckets.push(BucketInfo {
            name: bucket.name().unwrap_or_default().to_string(),
            creation_date: bucket.creation_date().unwrap().secs(),
        });
    }
    return Ok(my_buckets);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_buckets() {
        let result = get_buckets().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_buckets_error() {
        let result = create_error("Test Error".into(), "This is a test error".into());
        let error = result;
        assert_eq!(error.name, "Test Error");
        assert_eq!(error.message, "This is a test error");
    }
}
