use aws_sdk_s3::types::{Tag, Tagging};

use crate::libs::s3::{
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
        let tag = Tag::builder()
            .key(tag_key.clone())
            .value("true".to_string())
            .build();

        match tag {
            Ok(tag) => new_tags.push(tag),
            Err(err) => return Err(create_error("Failed to create tag".into(), err.to_string())),
        }
    }

    let _res = new_options.insert(new_tags);
    let set_tag = Tagging::builder().set_tag_set(new_options).build();

    match set_tag {
        Ok(tags) => {
            let aws_tags = client
                .put_object_tagging()
                .bucket(bucket)
                .key(key)
                .tagging(tags)
                .send()
                .await;

            match aws_tags {
                Ok(_) => Ok(()),
                Err(err) => Err(create_error(
                    "Aws Tagging operation failed".into(),
                    err.to_string(),
                )),
            }
        }
        Err(err) => {
            return Err(create_error(
                "Failed during tag creation".into(),
                err.to_string(),
            ))
        }
    }
}
