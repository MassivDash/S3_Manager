use aws_sdk_s3::{
    error::PutObjectTaggingError,
    model::{Tag, Tagging},
    output::PutObjectTaggingOutput,
    types::SdkError,
    Client,
};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::lib::s3::client::client::create_client;

#[tauri::command]
pub async fn set_all_tags(
    bucket: String,
    key: String,
    tag_keys: Vec<String>,
) -> Result<(), String> {
    let client: Client = create_client().await.unwrap();

    let mut new_options: Option<Vec<Tag>> = Option::None;
    let mut new_tags: Vec<Tag> = Vec::new();
    for tag_key in tag_keys {
        let new_tag: Tag = Tag::builder().key(tag_key).build();
        new_tags.push(new_tag)
    }

    let _res = new_options.insert(new_tags);
    let set_tag = Tagging::builder().set_tag_set(new_options).build();

    let aws_tags = client
        .put_object_tagging()
        .bucket(bucket)
        .key(key)
        .tagging(set_tag)
        .send()
        .await;

    if aws_tags.is_ok() {
        Ok(())
    } else {
        Err("Tagging call failed".into())
    }
}
