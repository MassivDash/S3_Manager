use std::error::Error;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;

pub async fn create_client() -> Result<Client, Box<dyn Error + Send + Sync>> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    Ok(client)
}
