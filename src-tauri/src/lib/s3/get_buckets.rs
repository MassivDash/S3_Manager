use aws_sdk_s3::{Client};
use cached::proc_macro::once;
use serde::{Deserialize, Serialize};

use super::client::create_client;

#[derive(Serialize, Deserialize, Clone)]
struct BucketObject {
    pub key: String,
    pub extension: String,
    pub size: i64,
    pub last_modified: i64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Bucket {
    pub name: String,
    pub files: Vec<BucketObject>,
    pub total_files: usize,
}

#[tauri::command]
pub async fn buckets() -> String {
    let client = create_client().await.unwrap();
    let resp = client.list_buckets().send().await;
    match resp {
        Ok(resp) => {
            let buckets = resp.buckets().unwrap();
            let mut my_buckets = Vec::new();
            for bucket in buckets {
                let files = get_objects(&client, bucket.name().unwrap_or_default()).await;
                my_buckets.push(Bucket {
                    name: bucket.name().unwrap_or_default().to_string(),
                    files: files.clone(),
                    total_files: files.len().clone(),
                });
            }
            return serde_json::to_string(&my_buckets).unwrap();
        }
        Err(err) => format!("{}", err.to_string()),
    }
}

// Cache the results, so we don't have to make a request every time.
#[once(time=900)] // 15 minutes
async fn get_objects(client: &Client, bucket: &str) -> Vec<BucketObject> {
    println!("{}", bucket);
    let resp = client.list_objects_v2().bucket(bucket).send().await;
    let mut files: Vec<BucketObject> = Vec::new();
    if let Ok(resp) = resp {
        let contents = resp.contents().unwrap();
        for object in contents {
                files.push(BucketObject {
                    key: object.key().unwrap_or_default().to_string(),
                    extension: object.key().unwrap_or_default().split(".").last().unwrap_or_default().to_string().to_lowercase(),
                    size: object.size(),
                    last_modified: object.last_modified().unwrap().clone().secs(),
                });
            }
        return files;
    } else {
        files
    }
}

