use aws_sdk_s3::types::{Tag, Tagging};

use crate::lib::s3::{
    client::client::create_client,
    utils::response_error::{create_error, ResponseError},
};

#[tauri::command]
pub async fn set_all_tags(
    bucket: String,
    key: String,
    tag_keys: Vec<String>,
) -> Result<(), ResponseError> {
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

    let mut new_options: Option<Vec<Tag>> = Option::None;
    let mut new_tags: Vec<Tag> = Vec::new();
    for tag_key in &tag_keys {
        let new_tag: Tag = Tag::builder().key(tag_key).value("").build();
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

    match aws_tags {
        Ok(_) => Ok(()),
        Err(err) => Err(create_error("Tagging failed".into(), err.to_string())),
    }
}
