use aws_sdk_s3::Client;
use cached::proc_macro::once;
use serde::{Deserialize, Serialize};
use itertools::Itertools;

use crate::lib::s3::client::client::create_client;


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
}

#[tauri::command]
pub async fn get_files() -> Vec<Bucket> {
    let client = create_client().await.unwrap();
    let resp = client.list_buckets().send().await.unwrap();
    let buckets = resp.buckets().unwrap();
    let mut my_buckets = Vec::new();
    for bucket in buckets {
        let files = get_objects(&client, bucket.name().unwrap_or_default()).await;
        let mut folders: Vec<BucketFolder> = Vec::new();
        let get_folders: Vec<String> = files.clone().into_iter().map(|x| x.folder.clone()).unique().collect();
        for folder in get_folders {
            let folder_files: Vec<BucketObject> = files.clone().into_iter().filter(|x| x.folder == folder).filter(|x| !x.name.ends_with("/")).collect();
            folders.push(BucketFolder {
                name: folder.clone(),
                files: folder_files.clone(),
                total_files: folder_files.clone().len(),
                total_size: folder_files.iter().fold(0, |acc, x| acc + x.size),
            });
        }
        my_buckets.push(Bucket {
            name: bucket.name().unwrap_or_default().to_string(),
            folders: folders,
            total_files: files.len().clone(),
        });
    }
    return my_buckets;
}

// Cache the results, so we don't have to make a request every time.
#[once(time = 900)] // 15 minutes
async fn get_objects(client: &Client, bucket: &str) -> Vec<BucketObject> {
    println!("{}", bucket);
    let resp = client.list_objects_v2().bucket(bucket).send().await;
    let mut files: Vec<BucketObject> = Vec::new();
    
    if let Ok(resp) = resp {
        let contents = resp.contents().unwrap();

        for object in contents {
            files.push(BucketObject {
                key: object.key().unwrap_or_default().to_string(),
                folder: object.key().unwrap_or_default().split("/").nth(0).unwrap_or_default().to_string(),
                name: object.key().unwrap().split("/").last().unwrap_or_default().to_string(),
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
        return files;
    } else {
        files
    }
}
