use aws_sdk_s3::{presigning::config::PresigningConfig, Client};
use std::{error::Error, time::Duration};

pub async fn get_presigned_url(
    client: &Client,
    bucket: &str,
    object: &str,
    expires_in: u64,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let expires_in = Duration::from_secs(expires_in);
    let presigned_request = client
        .get_object()
        .bucket(bucket)
        .key(object)
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;
    return Ok(presigned_request.uri().to_string());
}
