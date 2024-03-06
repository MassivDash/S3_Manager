use crate::libs::s3::{
    client::client::create_client,
    utils::response_error::{create_error, ResponseError},
};
use aws_sdk_s3::operation::get_object_tagging::GetObjectTaggingOutput;
use aws_sdk_s3::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ObjectTag {
    pub key: String,
    pub value: String,
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
    return Ok(resp);
}

#[tauri::command]
pub async fn get_all_tags(bucket: String, key: String) -> Result<Vec<ObjectTag>, ResponseError> {
    let client_call = create_client().await;
    let client = match client_call {
        Ok(instance) => instance,
        Err(err) => {
            println!("{}", err.to_string());
            return Err(create_error(
                "AWS Client Config error".into(),
                err.to_string(),
            ));
        }
    };
    let mut tags: Vec<ObjectTag> = Vec::new();

    let aws_tags = match get_tagging(&client, &bucket.to_string(), &key.to_string()).await {
        Ok(output) => output
            .tag_set()
            .into_iter()
            .map(|x| ObjectTag {
                key: x.key().to_string(),
                value: x.value().to_string(),
            })
            .collect::<Vec<ObjectTag>>(),
        Err(e) => return Err(create_error("tag input error".into(), e.to_string())),
    };

    if aws_tags.len() > 0 {
        tags = aws_tags
    }

    Ok(tags)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_all_tags() {
        let result = get_all_tags(
            "spaceout-backup".to_string(),
            "website/dockercli.jpeg".to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert!(result.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_get_empty_from_bad_call() {
        let result = get_all_tags(
            "spaceout-fuckup".to_string(),
            "website/dockercli.jpeg".to_string(),
        )
        .await;
        assert!(result.is_err())
    }
}
