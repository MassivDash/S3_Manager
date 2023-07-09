use aws_sdk_s3::Client;
use cached::proc_macro::once;
use serde::{Deserialize, Serialize};

use crate::libs::s3::client::client::create_client;
use crate::libs::s3::utils::presigned_url::get_presigned_url;
use crate::libs::s3::utils::response_error::{create_error, ResponseError};
use aws_sdk_s3::types::Object;
use std::error::Error;
use tokio_stream::StreamExt;

#[derive(Serialize, Deserialize, Clone)]
pub struct ImgBucketObject {
    pub key: String,
    pub name: String,
    pub url: String,
    pub size: i64,
    pub last_modified: i64,
    pub folder: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImgBucket {
    pub name: String,
    pub files: Vec<ImgBucketObject>,
    pub total_files: usize,
}

#[tauri::command]
#[once(time = 900)] // 15 minutes
pub async fn get_cached_movies() -> Vec<ImgBucket> {
    return get_all_movies().await.unwrap();
}

#[tauri::command]
pub async fn get_all_movies() -> Result<Vec<ImgBucket>, ResponseError> {
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
        let files_call = show_objects(&client, bucket.name().unwrap_or_default()).await;
        let files = match files_call {
            Ok(list) => list,
            Err(err) => {
                return Err(create_error(
                    "S3 object call failed".into(),
                    err.to_string(),
                ))
            }
        };

        my_buckets.push(ImgBucket {
            name: bucket.name().unwrap_or_default().to_string(),
            files: files.clone(),
            total_files: files.len().clone(),
        });
    }
    Ok(my_buckets)
}

async fn show_objects(
    client: &Client,
    bucket: &str,
) -> Result<Vec<ImgBucketObject>, Box<dyn Error>> {
    let mut resp = client
        .list_objects_v2()
        .bucket(bucket)
        .into_paginator()
        .send();
    let mut files: Vec<ImgBucketObject> = Vec::new();
    let mut objects: Vec<Object> = Vec::new();

    while let Some(page) = resp.next().await {
        let items = page?
            .contents()
            .unwrap()
            .iter()
            .map(|x| x.clone())
            .collect::<Vec<Object>>();
        objects.extend(items);
    }

    for object in objects {
        if check_if_file_is_movie(object.key().unwrap_or_default()) {
            files.push(ImgBucketObject {
                key: object.key().unwrap().to_string(),
                name: object
                    .key()
                    .unwrap()
                    .split("/")
                    .last()
                    .unwrap_or_default()
                    .to_string(),
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
                folder: object
                    .key()
                    .unwrap()
                    .split("/")
                    .nth(0)
                    .unwrap_or_default()
                    .to_string(),
            });
        }
    }
    return Ok(files);
}

fn check_if_file_is_movie(key: &str) -> bool {
    let extension = key.split('.').last().unwrap().to_lowercase().to_string();
    match extension.as_str() {
        "mov" | "mp4" | "webm" | "ogg" => true,
        _ => false,
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_all_movies() {
        let result = get_all_movies().await;
        assert!(result.is_ok());
        let bucket = result.unwrap();
        assert!(!bucket.is_empty());
        let mut found_bucket = false;
        for b in &bucket {
            if b.name == "lc-photobackup" {
                found_bucket = true;
                break;
            }
        }
        assert!(found_bucket);
    }
}
