use aws_sdk_s3::types::Object;
use aws_sdk_s3::Client;
use cached::proc_macro::once;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::lib::s3::client::client::create_client;
use crate::lib::s3::utils::presigned_url::get_presigned_url;
use crate::lib::s3::utils::response_error::{create_error, ResponseError};
use std::error::Error;
use tokio_stream::StreamExt;

#[derive(Serialize, Deserialize, Clone, Debug)]
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
pub async fn get_cached_images() -> Vec<ImgBucket> {
    return get_all_images().await.unwrap();
}

#[tauri::command]
pub async fn get_all_images() -> Result<Vec<ImgBucket>, ResponseError> {
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

        // Group files by the folder they are in

        let files = files
            .into_iter()
            .sorted_by(|a, b| a.folder.cmp(&b.folder))
            .group_by(|a| a.folder.clone())
            .into_iter()
            .map(|(_, group)| {
                group
                    .into_iter()
                    .sorted_by(|a, b| a.name.cmp(&b.name))
                    .collect::<Vec<ImgBucketObject>>()
            })
            .collect::<Vec<Vec<ImgBucketObject>>>()
            .concat();

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
    let mut tags = HashMap::new();
    tags.insert("karolina".to_owned(), "karolina".to_owned());

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
        if check_if_file_is_image(object.key().unwrap_or_default()) {
            let url = get_presigned_url(
                client,
                bucket,
                &object.key().unwrap_or_default().to_string(),
                36000, //15 minutes
            )
            .await
            .unwrap();

            let new_obj = ImgBucketObject {
                key: object.key().unwrap().to_string(),
                name: object
                    .key()
                    .unwrap()
                    .split("/")
                    .last()
                    .unwrap_or_default()
                    .to_string(),
                url: url,
                size: object.size(),
                last_modified: object.last_modified().unwrap().clone().secs(),
                folder: object
                    .key()
                    .unwrap()
                    .split("/")
                    .nth(0)
                    .unwrap_or_default()
                    .to_string(),
            };
            files.push(new_obj);
        }
    }
    return Ok(files);
}

fn check_if_file_is_image(key: &str) -> bool {
    let extension = key.split('.').last().unwrap().to_lowercase().to_string();
    match extension.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" => true,
        _ => false,
    }
}
