
use cached::proc_macro::once;
use aws_sdk_s3::Client;
use serde::{Deserialize, Serialize};

use crate::lib::s3::client::client::create_client;
use crate::lib::s3::utils::presigned_url::get_presigned_url;
use aws_sdk_s3::model::Object;
use tokio_stream::StreamExt;
use std::error::Error;

#[derive(Serialize, Deserialize, Clone)]
pub struct ImgBucketObject {
    pub key: String,
    pub name: String,
    pub url: String,
    pub size: i64,
    pub last_modified: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImgBucket {
    pub name: String,
    pub files: Vec<ImgBucketObject>,
    pub total_files: usize,
}

#[tauri::command]
#[once(time=900)] // 15 minutes
pub async fn get_cached_images() -> Vec<ImgBucket>{
    return get_all_images().await;
}

#[tauri::command]
pub async fn get_all_images() -> Vec<ImgBucket> {
    let client = create_client().await.unwrap();
    let resp = client.list_buckets().send().await.unwrap();
    let buckets = resp.buckets().unwrap();
    let mut my_buckets = Vec::new();
        for bucket in buckets {
                let files = show_objects(&client, bucket.name().unwrap_or_default()).await.unwrap();
                my_buckets.push(ImgBucket {
                    name: bucket.name().unwrap_or_default().to_string(),
                    files: files.clone(),
                    total_files: files.len().clone(),
                });
            }
    return my_buckets;
        
}


async fn show_objects(client: &Client, bucket: &str) -> Result<Vec<ImgBucketObject>, Box<dyn Error>> {
    let mut resp  = client.list_objects_v2().bucket(bucket).into_paginator().send();
    let mut files: Vec<ImgBucketObject> = Vec::new();
    let mut objects: Vec<Object> = Vec::new();


    while let Some(page) = resp.next().await {
        let items = page?.contents().unwrap().iter().map(|x| x.clone()).collect::<Vec<Object>>();
        objects.extend(items);
    }

        for object in objects {
            if check_if_file_is_image(object.key().unwrap_or_default()) {
                files.push(ImgBucketObject {
                    key: object.key().unwrap().to_string(),
                    name: object.key().unwrap().split("/").last().unwrap_or_default().to_string(),
                    url: get_presigned_url(
                        client,
                        bucket,
                        &object.key().unwrap_or_default().to_string(),
                        9000, //15 minutes
                    )
                    .await
                    .unwrap(),
                    size: object.size(),
                    last_modified: object.last_modified().unwrap().clone().secs(),
                });
            }
        }
        return Ok(files);
}

fn check_if_file_is_image(key: &str) -> bool {
    let extension = key.split('.').last().unwrap().to_lowercase().to_string();
    match extension.as_str() {
        "jpg" | "jpeg"| "png" | "gif" | "bmp" => true,
        _ => false,
    }
}
