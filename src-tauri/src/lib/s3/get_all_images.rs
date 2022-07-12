
use aws_sdk_s3::Client;
use serde::{Deserialize, Serialize};

use crate::lib::s3::presigned_url::get_presigned_url;
use crate::lib::s3::client::create_client;

#[derive(Serialize, Deserialize, Clone)]
pub struct ImgBucketObject {
    pub key: String,
    pub url: String,
    pub size: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImgBucket {
    pub name: String,
    pub files: Vec<ImgBucketObject>,
    pub total_files: usize,
}

#[tauri::command]
pub async fn get_all_images() -> Vec<ImgBucket> {
    let client = create_client().await.unwrap();
    let resp = client.list_buckets().send().await.unwrap();
    let buckets = resp.buckets().unwrap();
    let mut my_buckets = Vec::new();
        for bucket in buckets {
                let files = show_objects(&client, bucket.name().unwrap_or_default()).await;
                my_buckets.push(ImgBucket {
                    name: bucket.name().unwrap_or_default().to_string(),
                    files: files.clone(),
                    total_files: files.len().clone(),
                });
            }
    return my_buckets;
        
}

async fn show_objects(client: &Client, bucket: &str) -> Vec<ImgBucketObject> {
    let resp = client.list_objects_v2().bucket(bucket).send().await;
    let mut files: Vec<ImgBucketObject> = Vec::new();
    if let Ok(resp) = resp {
        let contents = resp.contents().unwrap();
        println!("{}", contents.len());
        for object in contents {
            if check_if_file_is_image(object.key().unwrap_or_default()) {
                files.push(ImgBucketObject {
                    key: object.key().unwrap_or_default().to_string(),
                    url: get_presigned_url(
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
