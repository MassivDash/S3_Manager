use crate::libs::s3::client::client::create_client;
use aws_sdk_s3::{
    types::{Delete, Object, ObjectIdentifier},
    Client,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tauri::Window;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilesToDelete {
    pub key: String,
    pub bucket_name: String,
}

#[tauri::command]
pub async fn delete_folders(window: Window, keys: Vec<FilesToDelete>) -> bool {
    let client = create_client().await.unwrap();
    for key in keys {
        remove_folder_with_contents(&client, &key.bucket_name, &key.key)
            .await
            .unwrap();
    }
    window.emit("event-resync", "delete successful").unwrap();
    return true;
}

async fn remove_folder_with_contents(
    client: &Client,
    bucket: &str,
    prefix: &str,
) -> Result<(), Box<dyn Error>> {
    let mut resp = client
        .list_objects_v2()
        .bucket(bucket)
        .prefix(prefix)
        .into_paginator()
        .send();

    let mut delete_objects: Vec<ObjectIdentifier> = vec![];
    let mut objects: Vec<Object> = Vec::new();

    while let Some(page) = resp.next().await {
        let items = page?
            .contents()
            .iter()
            .map(|x| x.clone())
            .collect::<Vec<Object>>();
        objects.extend(items);
    }

    for object in objects {
        let obj_id = ObjectIdentifier::builder().set_key(object.key).build();

        match obj_id {
            Ok(obj_id) => delete_objects.push(obj_id),
            Err(err) => {
                println!("{}", err.to_string());
            }
        }
    }

    let delete = Delete::builder().set_objects(Some(delete_objects)).build();

    client
        .delete_objects()
        .bucket(bucket)
        .delete(delete?)
        .send()
        .await?;

    println!("Objects deleted.");

    Ok(())
}
