use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use serde::{Deserialize, Serialize};

use crate::lib::s3::presigned_url::get_object;

#[derive(Serialize, Deserialize, Clone)]
pub struct BucketObject {
    pub key: String,
    pub url: String,
    pub size: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MyBucket {
    pub name: String,
    pub files: Vec<BucketObject>,
    pub total_files: usize,
}

#[tauri::command]
pub async fn buckets() -> String {
    // snippet-start:[s3.rust.client-client]
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    // snippet-end:[s3.rust.client-client]

    let resp = client.list_buckets().send().await;
    match resp {
        Ok(resp) => {
            let buckets = resp.buckets().unwrap();
            let mut my_buckets = Vec::new();
            for bucket in buckets {
                let files = show_objects(&client, bucket.name().unwrap_or_default()).await;
                my_buckets.push(MyBucket {
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

async fn show_objects(client: &Client, bucket: &str) -> Vec<BucketObject> {
    let resp = client.list_objects_v2().bucket(bucket).send().await;
    let mut files: Vec<BucketObject> = Vec::new();
    if let Ok(resp) = resp {
        let contents = resp.contents().unwrap();
        println!("{}", contents.len());
        for object in contents {
            if check_if_file_is_image(object.key().unwrap_or_default()) {
                files.push(BucketObject {
                    key: object.key().unwrap_or_default().to_string(),
                    url: get_object(
                        client,
                        bucket,
                        &object.key().unwrap_or_default().to_string(),
                        900,
                    )
                    .await
                    .unwrap(),
                    size: object.size(),
                });
            }
        }
        return files;
    } else {
        files
    }
}

fn check_if_file_is_image(key: &str) -> bool {
    println!("{}", key);
    let extension = key.split('.').last().unwrap().to_lowercase().to_string();
    match extension.as_str() {
        "jpg" | "jpeg"| "png" | "gif" | "bmp" => true,
        _ => false,
    }
}
