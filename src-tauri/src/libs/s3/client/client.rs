use std::error::Error;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;

pub async fn create_client() -> Result<Client, Box<dyn Error + Send + Sync>> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let mut config_builder = aws_config::from_env().region(region_provider);

    if use_custom_endpoint() {
        let endpoint = std::env::var("S3_CUSTOM_ENDPOINT_URL");
        match endpoint {
            Ok(endpoint) => {
                config_builder = config_builder.endpoint_url(endpoint);
            }
            Err(err) => return Err(err.into()),
        }
    };
    let config = config_builder.load().await;
    let client = Client::new(&config);
    Ok(client)
}

fn use_custom_endpoint() -> bool {
    std::env::var("S3_CUSTOM_ENDPOINT").is_ok()
}
