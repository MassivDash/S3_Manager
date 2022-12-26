use crate::lib::s3::client::client::create_client;
use aws_sdk_s3::output::GetObjectTaggingOutput;
use aws_sdk_s3::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ObjectTag {
    pub key: String,
    pub value: String,
}

pub async fn get_tag_amount(
    client: &Client,
    bucket: &str,
    object: &str,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let object = client
        .get_object()
        .bucket(bucket)
        .part_number(1)
        .response_content_type("json")
        .key(object)
        .send()
        .await?;

    println!("da fuck");
    return Ok(object.tag_count());
}

async fn get_tagging(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<GetObjectTaggingOutput, Box<dyn Error + Send + Sync>> {
    let resp = client
        .get_object_tagging()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    println!("{}, {}", &bucket, &key);
    return Ok(resp);
}

#[tauri::command]
pub async fn get_all_tags(bucket: String, key: String) -> Vec<ObjectTag> {
    let client = create_client().await.unwrap();
    let mut tags: Vec<ObjectTag> = Vec::new();

    let aws_tags = get_tagging(&client, &bucket.to_string(), &key.to_string())
        .await
        .unwrap()
        .tag_set()
        .unwrap()
        .into_iter()
        .map(|x| ObjectTag {
            key: x.key().unwrap().to_string(),
            value: x.value().unwrap().to_string(),
        })
        .collect::<Vec<ObjectTag>>();
    if aws_tags.len() > 0 {
        tags = aws_tags
    }

    return tags;
}