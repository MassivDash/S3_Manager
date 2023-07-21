use aws_sdk_s3::types::Object;
use aws_sdk_s3::Client;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio_stream::StreamExt;

use crate::libs::s3::{
    client::client::create_client,
    utils::response_error::{create_error, ResponseError},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BucketObject {
    pub key: String,
    pub name: String,
    pub folder: String,
    pub extension: String,
    pub size: i64,
    pub last_modified: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BucketFolder {
    pub name: String,
    pub files: Vec<BucketObject>,
    pub total_files: usize,
    pub total_size: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bucket {
    pub name: String,
    pub folders: Vec<BucketFolder>,
    pub total_files: usize,
    pub total_size: i64,
}

#[tauri::command]
pub async fn get_files() -> Result<Vec<Bucket>, ResponseError> {
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
        let files_call = get_objects(&client, bucket.name().unwrap_or_default()).await;
        let files = match files_call {
            Ok(list) => list,
            Err(err) => {
                println!("{}", err.to_string());
                return Err(create_error(
                    "S3 object call failed".into(),
                    err.to_string(),
                ));
            }
        };

        let mut folders: Vec<BucketFolder> = Vec::new();
        let get_folders: Vec<String> = files
            .clone()
            .into_iter()
            .map(|x| x.folder.clone())
            .unique()
            .collect();
        for folder in get_folders {
            let folder_files: Vec<BucketObject> = files
                .clone()
                .into_iter()
                .filter(|x| x.folder == folder)
                .filter(|x| !x.key.ends_with("/"))
                .collect();
            folders.push(BucketFolder {
                name: folder.clone(),
                files: folder_files.clone(),
                total_files: folder_files.clone().len(),
                total_size: folder_files.iter().fold(0, |acc, x| acc + x.size),
            });
        }
        my_buckets.push(Bucket {
            name: bucket.name().unwrap_or_default().to_string(),
            folders: folders.clone(),
            total_files: files.len().clone(),
            total_size: folders.iter().fold(0, |acc, x| acc + x.total_size),
        });
    }
    return Ok(my_buckets);
}

async fn get_objects(client: &Client, bucket: &str) -> Result<Vec<BucketObject>, Box<dyn Error>> {
    let mut resp = client
        .list_objects_v2()
        .bucket(bucket)
        .into_paginator()
        .send();
    let mut files: Vec<BucketObject> = Vec::new();
    let mut objects: Vec<Object> = Vec::new();

    while let Some(page) = resp.next().await {
        let items = page?
            .contents()
            .unwrap_or_default()
            .iter()
            .map(|x| x.clone())
            .collect::<Vec<Object>>();
        objects.extend(items);
    }

    for object in objects {
        files.push(BucketObject {
            key: object.key().unwrap_or_default().to_string(),
            folder: object
                .key()
                .unwrap_or_default()
                .split("/")
                .nth(0)
                .unwrap_or_default()
                .to_string(),
            name: object
                .key()
                .unwrap()
                .split("/")
                .last()
                .unwrap_or_default()
                .to_string(),
            extension: object
                .key()
                .unwrap_or_default()
                .split(".")
                .last()
                .unwrap_or_default()
                .to_string()
                .to_lowercase(),
            size: object.size(),
            last_modified: object.last_modified().unwrap().clone().secs(),
        });
    }
    return Ok(files);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_objects() {
        let result = get_files().await;
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
